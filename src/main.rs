use libp2p::{
    core::Multiaddr,
    futures::StreamExt,
    gossipsub,
    identify,
    kad,
    noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux,
    quic,
    StreamProtocol,
};

use std::error::Error;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use tokio::io::{self, AsyncBufReadExt};

use uuid::Uuid;
use chrono::Utc;
use prost::Message;

mod proto;
use proto::peerboard::v1::PeerBoardMessage;

// =========================================================
// CONFIG
// =========================================================

const CHAT_TOPIC: &str = "peerboard/v1/general";

// =========================================================
// BEHAVIOUR
// =========================================================

#[derive(NetworkBehaviour)]
struct ChatBehaviour {
    gossipsub: gossipsub::Behaviour,
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
    identify: identify::Behaviour,
}

// =========================================================
// IDENTITY
// =========================================================

fn default_identity_path() -> Result<PathBuf, Box<dyn Error>> {
    let home = env::var_os("HOME").ok_or("HOME environment variable not set")?;
    let mut path = PathBuf::from(home);
    path.push(".peerboard");
    path.push("identity.key");
    Ok(path)
}

fn parse_identity_path() -> Result<PathBuf, Box<dyn Error>> {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--identity-file" {
            if let Some(path) = args.next() {
                return Ok(PathBuf::from(path));
            }
            return Err("missing value for --identity-file".into());
        }
        if let Some(value) = arg.strip_prefix("--identity-file=") {
            return Ok(PathBuf::from(value));
        }
    }
    default_identity_path()
}

fn load_or_generate_identity(path: &Path) -> Result<libp2p::identity::Keypair, Box<dyn Error>> {
    if path.exists() {
        let bytes = fs::read(path)?;
        let keypair = libp2p::identity::Keypair::from_protobuf_encoding(&bytes)?;
        return Ok(keypair);
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let keypair = libp2p::identity::Keypair::generate_ed25519();
    let encoded = keypair.to_protobuf_encoding()?;
    fs::write(path, encoded)?;
    Ok(keypair)
}

// =========================================================
// PROTO HELPERS
// =========================================================

fn encode_message(
    peer_id: &str,
    topic: &str,
    content: String,
    nickname: String,
) -> Result<Vec<u8>, Box<dyn Error>> {

    if content.len() > 4096 {
        return Err("content exceeds 4096 bytes".into());
    }

    if nickname.len() > 32 {
        return Err("nickname exceeds 32 bytes".into());
    }

    let msg = PeerBoardMessage {
        peer_id: peer_id.to_string(),
        topic: topic.to_string(),
        content,
        timestamp: Utc::now().timestamp(),
        message_id: Uuid::new_v4().to_string(),
        nickname,
    };

    let mut buf = Vec::new();
    msg.encode(&mut buf)?;
    Ok(buf)
}

fn decode_message(bytes: &[u8]) -> Result<PeerBoardMessage, prost::DecodeError> {
    PeerBoardMessage::decode(bytes)
}

// =========================================================
// MAIN
// =========================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let identity_path = parse_identity_path()?;
    let local_key = load_or_generate_identity(&identity_path)?;

    println!("Using identity file: {}", identity_path.display());

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key.clone())
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_behaviour(|key| {

            let peer_id = key.public().to_peer_id();

            let kad_config = kad::Config::new(
                StreamProtocol::new("/peerboard/kad/1.0.0")
            );

            let store = kad::store::MemoryStore::new(peer_id);

            let kademlia = kad::Behaviour::with_config(
                peer_id,
                store,
                kad_config,
            );

            let identify = identify::Behaviour::new(
                identify::Config::new(
                    "/peerboard/identify/1.0.0".into(),
                    key.public(),
                )
            );

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
        .with_swarm_config(|cfg| {
            cfg.with_idle_connection_timeout(Duration::from_secs(60))
        })
        .build();

    let local_peer_id = *swarm.local_peer_id();
    println!("Local peer id: {local_peer_id}");

    // =========================================================
    // GOSSIPSUB TOPIC
    // =========================================================

    let topic = gossipsub::IdentTopic::new(CHAT_TOPIC);

    swarm.behaviour_mut()
        .gossipsub
        .subscribe(&topic)?;

    println!("Subscribed to topic: {CHAT_TOPIC}");

    // =========================================================
    // LISTEN
    // =========================================================

    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // =========================================================
    // BOOTSTRAP
    // =========================================================

    let bootstrap_peer_id = "12D3KooWCvwqT3JUzVQczCvAVFa9EGzNqjHHSMVHVhm3RVyscCNY".parse()?;
    let bootstrap_addr: Multiaddr = "/ip4/170.64.177.57/tcp/8000".parse()?;

    swarm.behaviour_mut()
        .kademlia
        .add_address(&bootstrap_peer_id, bootstrap_addr.clone());

    swarm.dial(bootstrap_addr.clone())?;

    // =========================================================
    // STDIN
    // =========================================================

    let mut stdin = io::BufReader::new(io::stdin()).lines();
    println!("Enter chat messages:");

    loop {
        tokio::select! {

            Ok(Some(line)) = stdin.next_line() => {

                let peer_id = swarm.local_peer_id().to_string();
                let nickname = "ma".to_string();

                let encoded = match encode_message(
                    &peer_id,
                    CHAT_TOPIC,
                    line,
                    nickname,
                ) {
                    Ok(data) => data,
                    Err(e) => {
                        println!("Encode error: {e}");
                        continue;
                    }
                };

                if let Err(err) = swarm.behaviour_mut()
                    .gossipsub
                    .publish(topic.clone(), encoded)
                {
                    println!("Publish error: {err:?}");
                }
            }

            event = swarm.select_next_some() => match event {

                // ---------------- Listening ----------------
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {address}");
                }

                // ---------------- Bootstrap ----------------
                SwarmEvent::ConnectionEstablished { peer_id, .. }
                    if peer_id == bootstrap_peer_id =>
                {
                    println!("Connected to bootstrap. Running Kademlia bootstrap...");

                    let _ = swarm.behaviour_mut().kademlia.bootstrap();

                    swarm.behaviour_mut()
                        .kademlia
                        .get_closest_peers(local_peer_id);
                }

                // ---------------- Identify ----------------
                SwarmEvent::Behaviour(ChatBehaviourEvent::Identify(
                    identify::Event::Received { peer_id, info, .. }
                )) => {
                    for addr in info.listen_addrs {
                        swarm.behaviour_mut()
                            .kademlia
                            .add_address(&peer_id, addr);
                    }
                }

                // ---------------- GossipSub ----------------
                SwarmEvent::Behaviour(ChatBehaviourEvent::Gossipsub(
                    gossipsub::Event::Message {
                        propagation_source,
                        message_id,
                        message,
                    }
                )) => {
                    match decode_message(&message.data) {
                        Ok(msg) => {
                            println!(
                                "\n[TOPIC: {}]\nFROM: {}\nNICK: {}\nMSG_ID: {}\nTIME: {}\nCONTENT:\n{}\n",
                                msg.topic,
                                propagation_source,
                                msg.nickname,
                                msg.message_id,
                                msg.timestamp,
                                msg.content
                            );
                        }
                        Err(e) => {
                            println!(
                                "Invalid protobuf from {} (msg_id {:?}): {}",
                                propagation_source,
                                message_id,
                                e
                            );
                        }
                    }
                }

                // ---------------- Kademlia ----------------
                SwarmEvent::Behaviour(ChatBehaviourEvent::Kademlia(
                    kad::Event::OutboundQueryProgressed { result, .. }
                )) => {
                    if let kad::QueryResult::GetClosestPeers(Ok(ok)) = result {
                        for peer in ok.peers {
                            println!("Discovered peer: {:?}", peer.peer_id);
                        }
                    }
                }

                _ => {}
            }
        }
    }
}