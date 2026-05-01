pub mod cli;
pub mod server;
pub mod state;

use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (tx_to_swarm, _rx_to_swarm) = mpsc::channel(32);
    let (tx_to_client, _rx_to_client) = broadcast::channel(32);

    let state = Arc::new(state::AppState {
        tx_to_swarm,
        tx_to_client,
    });

    server::start_server(state).await;
    Ok(())
}