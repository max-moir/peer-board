pub mod server;
pub mod state;
pub mod swarm_runner;  // Import swarm runner

use crate::app::{server::start_server, swarm_runner::run_swarm};
use crate::core::identity::load_or_generate_identity;
use std::sync::Arc;
use tokio::sync::{mpsc, broadcast};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Load the identity for the swarm
    let identity_path = crate::core::identity::parse_identity_path()?;
    let key = load_or_generate_identity(&identity_path)?;

    // Create the channels for communication between React, Axum, and libp2p
    let (tx_to_swarm, rx_to_swarm) = mpsc::channel::<String>(100);
    let (tx_to_client, _rx_to_client) = broadcast::channel::<String>(100);

    // Create shared state (AppState)
    let state = Arc::new(state::AppState {
        tx_to_swarm,
        tx_to_client: tx_to_client.clone(),
    });

    // Spawn the swarm runtime as a background task
    tokio::spawn(async move {
        if let Err(err) = run_swarm(rx_to_swarm, tx_to_client, key).await {
            eprintln!("swarm task error: {err}");
        }
    });

    // Start the Axum WebSocket server
    start_server(state).await;

    Ok(())
}