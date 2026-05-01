use libp2p::{
    core::Multiaddr,
    futures::StreamExt,
    gossipsub,
    identify,
    kad,
    noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, quic,
    StreamProtocol,
};
use std::error::Error;
use std::time::Duration;
use tokio::io::{self, AsyncBufReadExt};

#[derive(NetworkBehaviour)]
struct ChatBehaviour {
    gossipsub: gossipsub::Behaviour,
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
    identify: identify::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default)?
        .with_quic()
        .with_behaviour(|key| {
            let peer_id = key.public().to_peer_id();

            // ---------------- Kademlia ----------------
            let mut kad_config = kad::Config::new(
                StreamProtocol::new("/peerboard/kad/1.0.0")
            );

            let store = kad::store::MemoryStore::new(peer_id);

            let kademlia = kad::Behaviour::with_config(
                peer_id,
                store,
                kad_config,
            );

            // ---------------- Identify ----------------
            let identify = identify::Behaviour::new(
                identify::Config::new(
                    "/peerboard/identify/1.0.0".into(),
                    key.public(),
                )
            );

            // ---------------- Gossipsub ----------------
            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub::Config::default(),
            )?;

            Ok(ChatBehaviour {
                gossipsub,
                kademlia,
                identify,
            })
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    let local_peer_id = *swarm.local_peer_id();
    println!("Local peer id: {local_peer_id}");

    // =========================================================
    // TOPIC: peerboard/v1/general
    // =========================================================
    let topic = gossipsub::IdentTopic::new("peerboard/v1/general");

    swarm.behaviour_mut()
        .gossipsub
        .subscribe(&topic)?;

    println!("Subscribed to topic: peerboard/v1/general");

    // ---------------- Listen ----------------
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // =========================================================
    // BOOTSTRAP CONFIG
    // =========================================================
    let bootstrap_peer_id = "12D3KooWCvwqT3JUzVQczCvAVFa9EGzNqjHHSMVHVhm3RVyscCNY".parse()?;
    let bootstrap_addr: Multiaddr = "/ip4/170.64.177.57/tcp/8000".parse()?;

    swarm.behaviour_mut()
        .kademlia
        .add_address(&bootstrap_peer_id, bootstrap_addr.clone());

    swarm.dial(bootstrap_addr.clone())?;

    // ---------------- stdin ----------------
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    println!("Enter chat messages:");

    loop {
        tokio::select! {
            Ok(Some(line)) = stdin.next_line() => {
                if let Err(err) =
                    swarm.behaviour_mut().gossipsub.publish(topic.clone(), line.into_bytes())
                {
                    println!("Publish error: {err:?}");
                }
            }

            event = swarm.select_next_some() => match event {

                // ---------------- Listening ----------------
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {address}");
                }

                // ---------------- Bootstrap connection ----------------
                SwarmEvent::ConnectionEstablished { peer_id, .. }
                    if peer_id == bootstrap_peer_id =>
                {
                    println!("Connected to bootstrap. Running Kademlia bootstrap...");

                    swarm.behaviour_mut().kademlia.bootstrap()?;

                    swarm.behaviour_mut()
                        .kademlia
                        .get_closest_peers(local_peer_id);
                }

                // ---------------- Identify (CRITICAL) ----------------
                SwarmEvent::Behaviour(ChatBehaviourEvent::Identify(
                    identify::Event::Received { peer_id, info, .. }
                )) => {
                    for addr in info.listen_addrs {
                        swarm.behaviour_mut()
                            .kademlia
                            .add_address(&peer_id, addr);
                    }
                }

                // ---------------- GossipSub messages (DEBUG VIEW) ----------------
                SwarmEvent::Behaviour(ChatBehaviourEvent::Gossipsub(
                    gossipsub::Event::Message {
                        propagation_source,
                        message_id,
                        message,
                    }
                )) => {
                    println!(
                        "\n[TOPIC: {}]\nFROM: {}\nID: {}\nMSG: {}\n",
                        message.topic.as_str(),
                        propagation_source,
                        message_id,
                        String::from_utf8_lossy(&message.data)
                    );
                }

                // ---------------- Kademlia peer discovery ----------------
                SwarmEvent::Behaviour(ChatBehaviourEvent::Kademlia(
                    kad::Event::OutboundQueryProgressed { result, .. }
                )) => {
                    if let kad::QueryResult::GetClosestPeers(Ok(ok)) = result {
                        for peer in ok.peers {
                            println!("Dialing discovered peer: {:?}", peer.peer_id);
                            swarm.dial(peer.peer_id).ok();
                        }
                    }
                }

                // ---------------- Kademlia debug ----------------
                SwarmEvent::Behaviour(ChatBehaviourEvent::Kademlia(event)) => {
                    println!("Kademlia event: {event:?}");
                }

                _ => {}
            }
        }
    }
}