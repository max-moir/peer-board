use std::time::Duration;

use libp2p::{
    identity::Keypair,
    tcp, noise, yamux,
    gossipsub, identify, kad,
    StreamProtocol,
    swarm::Swarm,
};

use libp2p::swarm::NetworkBehaviour;

#[derive(NetworkBehaviour)]
pub struct ChatBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
    pub identify: identify::Behaviour,
}

pub fn build_swarm(
    key: Keypair,
) -> Result<Swarm<ChatBehaviour>, Box<dyn std::error::Error>> {

    let peer_id = key.public().to_peer_id();

    let store = kad::store::MemoryStore::new(peer_id);
    let kad_cfg = kad::Config::new(StreamProtocol::new("/peerboard/kad/1.0.0"));

    let kademlia = kad::Behaviour::with_config(peer_id, store, kad_cfg);

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

    let behaviour = ChatBehaviour {
        gossipsub,
        kademlia,
        identify,
    };

    let swarm = libp2p::SwarmBuilder::with_existing_identity(key)
        .with_tokio()
        .with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default)?
        .with_quic()
        .with_behaviour(|_| Ok(behaviour))?
        .with_swarm_config(|cfg| {
            cfg.with_idle_connection_timeout(Duration::from_secs(60))
        })
        .build();

    Ok(swarm)
}