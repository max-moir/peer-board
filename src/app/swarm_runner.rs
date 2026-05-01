use tokio::sync::mpsc;
use futures_util::StreamExt;
use libp2p::{gossipsub, swarm::SwarmEvent};

use crate::{
    core::{
        swarm::{build_swarm, ChatBehaviourEvent},
        message::{encode_message, decode_and_validate_message, MessageDedup},
    },
};

const CHAT_TOPIC: &str = "peerboard/v1/general";

pub async fn run_swarm(
    mut rx: mpsc::Receiver<String>,
    tx_to_client: tokio::sync::broadcast::Sender<String>,
    key: libp2p::identity::Keypair,
) -> Result<(), Box<dyn std::error::Error>> {
    // Build the libp2p swarm
    let mut swarm = build_swarm(key.clone())?;
    let local_peer = *swarm.local_peer_id();

    let dedup = MessageDedup::default();
    let topic = gossipsub::IdentTopic::new(CHAT_TOPIC);

    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    // Listen on both TCP and QUIC
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;

    // Simulate a bootstrap peer
    let bootstrap_peer_id = "12D3KooWCvwqT3JUzVQczCvAVFa9EGzNqjHHSMVHVhm3RVyscCNY"
        .parse()?;
    let bootstrap_addr = "/ip4/170.64.177.57/tcp/8000".parse::<libp2p::Multiaddr>()?;
    swarm.behaviour_mut()
        .kademlia
        .add_address(&bootstrap_peer_id, bootstrap_addr.clone());

    swarm.dial(bootstrap_addr)?;

    // Set up the message channel for React → Swarm

    loop {
        tokio::select! {
            // Receive messages from the WebSocket clients and send them to the swarm
            Some(line) = rx.recv() => {
                let data = encode_message(
                    &local_peer.to_string(),
                    CHAT_TOPIC,
                    line,
                    "ma".to_string(),
                )?;

                // Publish to the gossip network
                let _ = swarm.behaviour_mut()
                    .gossipsub
                    .publish(topic.clone(), data);
            }

            // Process events from the libp2p swarm
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
                        let formatted = format!(
                            "[{}] {}: {}",
                            propagation_source,
                            msg.nickname,
                            msg.content
                        );

                        // Send the message to the WebSocket clients via the broadcast channel
                        let _ = tx_to_client.send(formatted);
                    }
                }
                _ => {}
            }
        }
    }
}