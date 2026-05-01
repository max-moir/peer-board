use std::sync::Arc;
use tokio::sync::{mpsc, broadcast};

/// Messages from React → Swarm
pub type ToSwarmTx = mpsc::Sender<String>;
pub type ToSwarmRx = mpsc::Receiver<String>;

/// Messages from Swarm → all React clients
pub type ToClientTx = broadcast::Sender<String>;
pub type ToClientRx = broadcast::Receiver<String>;

#[derive(Clone)]
pub struct AppState {
    pub tx_to_swarm: ToSwarmTx,
    pub tx_to_client: ToClientTx,
}

pub type SharedState = Arc<AppState>;