use std::sync::Arc;
use tokio::sync::{mpsc, broadcast};
use crate::app::ws_protocol::{WsIncoming, WsOutgoing};
use crate::core::db::MessageStore;

pub type ToSwarmTx = mpsc::Sender<WsIncoming>;
pub type ToClientTx = broadcast::Sender<WsOutgoing>;

#[derive(Clone)]
pub struct AppState {
    pub tx_to_swarm: ToSwarmTx,
    pub tx_to_client: ToClientTx,
    pub db: Arc<MessageStore>,
}

pub type SharedState = Arc<AppState>;