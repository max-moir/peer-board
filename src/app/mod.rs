pub mod server;
pub mod state;
pub mod swarm_runner;  
pub mod ws_protocol;
use crate::core::db::MessageStore;

use crate::app::{server::start_server, swarm_runner::run_swarm, ws_protocol::{WsIncoming, WsOutgoing}};
use crate::core::identity::load_or_generate_identity;
use std::sync::Arc;
use tokio::sync::{mpsc, broadcast};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let identity_path = crate::core::identity::parse_identity_path()?;
    let key = load_or_generate_identity(&identity_path)?;

    let (tx_to_swarm, rx_to_swarm) = mpsc::channel::<WsIncoming>(100);
    let (tx_to_client, _rx_to_client) = broadcast::channel::<WsOutgoing>(100);
    let db = Arc::new(MessageStore::new("chat.db")?);

    let state = Arc::new(state::AppState {
        tx_to_swarm,
        tx_to_client: tx_to_client.clone(),
        db: db.clone(),
    });

    tokio::spawn(async move {
        if let Err(err) = run_swarm(rx_to_swarm, tx_to_client, key, db.clone()).await {
            eprintln!("swarm task error: {err}");
        }
    });

    start_server(state).await;

    Ok(())
}