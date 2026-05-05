use tokio::sync::mpsc;
use futures_util::StreamExt;
use libp2p::{gossipsub, swarm::SwarmEvent, rendezvous, Multiaddr};
use serde_json::json;
use std::time::Duration;

use crate::{
    core::{
        swarm::{build_swarm, ChatBehaviourEvent},
        message::{encode_message, decode_and_validate_message, MessageDedup},
        db::{MessageStore, Message as DbMessage, current_timestamp},
    },
    app::ws_protocol::{WsIncoming, WsOutgoing}
};


const CHAT_TOPIC: &str = "peerboard/v1/general";
const CHALLENGE_NS: &str = "peerboard/challenge/seeking";
const BATTLESHIP_PATH: &str = "peerboard/challenge/1.0.0";
const BOOTSTRAP_NODE: &str = "/ip4/170.64.177.57/tcp/8000/p2p/12D3KooWCvwqT3JUzVQczCvAVFa9EGzNqjHHSMVHVhm3RVyscCNY";

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

    let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse()?;
    swarm.listen_on(listen_addr.clone())?;
    swarm.add_external_address(listen_addr);

    let bootstrap_addr: Multiaddr = BOOTSTRAP_NODE.parse()?;
    let bootstrap_peer_id = "12D3KooWCvwqT3JUzVQczCvAVFa9EGzNqjHHSMVHVhm3RVyscCNY"
        .parse()?;
                        
    swarm.behaviour_mut()
        .kademlia
        .add_address(&bootstrap_peer_id, bootstrap_addr.clone());


    swarm.dial(bootstrap_addr.clone()).unwrap();


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

                    WsIncoming::local_id_req { } => {
                        let outgoing = WsOutgoing::local_id {
                            id: local_peer.to_string(),
                        };

                        let _ = tx_to_client.send(outgoing);

                    },
                    WsIncoming::register_for_game{ nickname }  => {
                        use rendezvous::Namespace;
                        println!("seeking");

                        let ns = Namespace::new("peerboard/challenge/seeking".to_string()).unwrap();

                        let _ = swarm.behaviour_mut().rendezvous.register(ns, bootstrap_peer_id, None);
                    },

                    WsIncoming::discover{} => {
                        let _ = swarm.behaviour_mut()
                            .rendezvous
                            .discover(
                                Some(rendezvous::Namespace::from_static("peerboard/challenge/seeking")),
                                None,
                                Some(10),
                                bootstrap_peer_id,
                            );

                        println!("discovering");

                    }

                    WsIncoming::unregister_for_game{ }  => {
                        println!("Not seeking");
                        let _ = swarm.behaviour_mut()
                            .rendezvous
                            .unregister(
                                rendezvous::Namespace::from_static("peerboard/challenge/seeking"),
                                bootstrap_peer_id,
                        );
                    },

                    WsIncoming::send_challenge{ peer_id } => {
                        println!("{}", peer_id);


                    },

                    _ => {}
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



                SwarmEvent::Behaviour(ChatBehaviourEvent::Rendezvous(event)) => {
                    use libp2p::rendezvous::client::Event;

                    match event {
                        Event::Registered { namespace, ttl, .. } => {
                            println!("Registered on namespace {:?} for {} seconds", namespace, ttl);
                        },
                        Event::Discovered { rendezvous_node, registrations, cookie } => {
                            println!("Discovered {} registrations from rendezvous node {}", registrations.len(), rendezvous_node);

                            let peer_ids: Vec<String> = registrations
                                .iter()
                                .map(|reg| reg.record.peer_id().to_string())
                                .collect();


                            let outgoing = WsOutgoing::discover_response {
                                peers: peer_ids, 
                            };

                            let _ = tx_to_client.send(outgoing);
                        },
                        _ => {}
                    }
                },
        _ => {}


            }
        }
    }
}