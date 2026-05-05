use tokio::sync::mpsc;
use futures_util::StreamExt;
use libp2p::{gossipsub, swarm::SwarmEvent};
use serde_json::json;

use crate::{
    core::{
        swarm::{build_swarm, ChatBehaviourEvent},
        message::{encode_message, decode_and_validate_message, MessageDedup},
        db::{MessageStore, Message as DbMessage, current_timestamp},
    },
    app::ws_protocol::{WsIncoming, WsOutgoing}
};


const CHAT_TOPIC: &str = "peerboard/v1/general";

pub async fn run_swarm(
    mut rx: mpsc::Receiver<WsIncoming>,
    tx_to_client: tokio::sync::broadcast::Sender<WsOutgoing>,
    key: libp2p::identity::Keypair,
    db: std::sync::Arc<MessageStore>
) -> Result<(), Box<dyn std::error::Error>> {

    // Build the swarm
    let mut swarm = build_swarm(key.clone())?;
    let local_peer = *swarm.local_peer_id();

    let dedup = MessageDedup::default();
    let topic = gossipsub::IdentTopic::new(CHAT_TOPIC);

    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    // Listen on both TCP and QUIC
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;

    let bootstrap_peer_id = "12D3KooWCvwqT3JUzVQczCvAVFa9EGzNqjHHSMVHVhm3RVyscCNY"
        .parse()?;
    let bootstrap_addr = "/ip4/170.64.177.57/tcp/8000".parse::<libp2p::Multiaddr>()?;
    swarm.behaviour_mut()
        .kademlia
        .add_address(&bootstrap_peer_id, bootstrap_addr.clone());

    swarm.dial(bootstrap_addr)?;


    loop {
        tokio::select! {
            // Receive from websockeit
            Some(msg) = rx.recv() => {
                match msg {

                    WsIncoming::send_message { topic, nickname, content } => {
                        let full_topic = format!("peerboard/v1/{}", topic);

                        let data = encode_message(
                            &local_peer.to_string(),
                            &full_topic,
                            content.clone(),
                            nickname.clone(),
                        )?;

                        let db_msg = DbMessage {
                            peer_id: local_peer.to_string(),
                            message_id: format!("local-{}", current_timestamp()),
                            topic: topic.clone(),
                            nickname,
                            content,
                            timestamp: current_timestamp(),
                        };

                        let _ = db.insert_message(&db_msg);

                        let gossip_topic = gossipsub::IdentTopic::new(&full_topic);

                        let _ = swarm.behaviour_mut()
                            .gossipsub
                            .publish(gossip_topic, data);
                    }

                    WsIncoming::subscribe_topic { topic } => {
                        let full_topic = format!("peerboard/v1/{}", topic);

                        let _ = swarm.behaviour_mut()
                            .gossipsub
                            .subscribe(&gossipsub::IdentTopic::new(&full_topic));
                    }

                    WsIncoming::unsubscribe_topic { topic } => {
                        let full_topic = format!("peerboard/v1/{}", topic);
                        let _ = swarm.behaviour_mut()
                            .gossipsub
                            .unsubscribe(&gossipsub::IdentTopic::new(&full_topic));
                    }

                    WsIncoming::history { .. } => {
                        // This should be handled in server
                    }
                }

            }

            // Process events from the swarm
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(ChatBehaviourEvent::Gossipsub(
                    libp2p::gossipsub::Event::Message {
                        propagation_source,
                        message,
                        ..
                    }
                )) => {
                    // Decode and validate the message, then send it to the WebSocket clients
                    if let Some(msg) = decode_and_validate_message(&message.data, &dedup) {

                        let outgoing = WsOutgoing::message {
                            peer_id: msg.peer_id.clone(),
                            topic: msg.topic.clone(),
                            content: msg.content.clone(),
                            timestamp: current_timestamp() as i64,
                            message_id: msg.message_id.clone(),
                            nickname: msg.nickname.clone(),
                        };

                        let _ = tx_to_client.send(outgoing);

                        let db_msg = DbMessage {
                            peer_id: msg.peer_id.clone(),
                            topic: msg.topic.clone(),
                            content: msg.content.clone(),
                            timestamp: current_timestamp() as i64,
                            message_id: msg.message_id.clone(),
                            nickname: msg.nickname.clone(),
                        };

                        let _ = db.insert_message(&db_msg);
                    }
                }
                _ => {}
            }
        }
    }
}