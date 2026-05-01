use tokio::io::{self, AsyncBufReadExt};

use libp2p::{
    gossipsub,
    swarm::SwarmEvent,
    futures::StreamExt,
};

use crate::core::{
    identity::parse_identity_path,
    identity::load_or_generate_identity,
    message::{encode_message, decode_message},
    swarm::build_swarm,
    swarm::ChatBehaviourEvent,
};

const CHAT_TOPIC: &str = "peerboard/v1/general";

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {

    let identity_path = parse_identity_path()?;
    let key = load_or_generate_identity(&identity_path)?;

    let mut swarm = build_swarm(key.clone())?;

    let local_peer = *swarm.local_peer_id();
    println!("local peer: {local_peer}");

    // ---------------- GOSSIP ----------------
    let topic = gossipsub::IdentTopic::new(CHAT_TOPIC);
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    // ---------------- LISTEN ----------------
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;

    // ---------------- BOOTSTRAP ----------------
    let bootstrap_peer_id = "12D3KooWCvwqT3JUzVQczCvAVFa9EGzNqjHHSMVHVhm3RVyscCNY"
        .parse()?;

    let bootstrap_addr =
        "/ip4/170.64.177.57/tcp/8000"
            .parse::<libp2p::Multiaddr>()?;

    swarm.behaviour_mut()
        .kademlia
        .add_address(&bootstrap_peer_id, bootstrap_addr.clone());

    swarm.dial(bootstrap_addr)?;

    // ---------------- STDIN ----------------
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    loop {
        tokio::select! {

            Ok(Some(line)) = stdin.next_line() => {

                let data = encode_message(
                    &local_peer.to_string(),
                    CHAT_TOPIC,
                    line,
                    "ma".to_string(),
                )?;

                let _ = swarm.behaviour_mut()
                    .gossipsub
                    .publish(topic.clone(), data);
            }

            event = swarm.select_next_some() => match event {

                // ---------------- CHAT MESSAGE ----------------
                SwarmEvent::Behaviour(ChatBehaviourEvent::Gossipsub(
                    libp2p::gossipsub::Event::Message {
                        propagation_source,
                        message,
                        ..
                    }
                )) => {
                    match decode_message(&message.data) {
                        Ok(msg) => {
                            println!(
                                "[{}] {}: {}",
                                propagation_source,
                                msg.nickname,
                                msg.content
                            );
                        }
                        Err(_) => {
                            println!(
                                "[{}] <invalid message>",
                                propagation_source
                            );
                        }
                    }
                }


                _ => {}
            }
        }
    }
}