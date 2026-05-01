use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade, Message},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::app::state::SharedState;

pub async fn start_server(state: SharedState) {
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("WebSocket running on ws://127.0.0.1:3001/ws");

    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: SharedState) {
    let mut rx = state.tx_to_client.subscribe();

    loop {
        tokio::select! {

            // React → Swarm
            Some(Ok(Message::Text(text))) = socket.recv() => {
                println!("received from browser: {}", text);
                let _ = state.tx_to_swarm.send(text.clone()).await;
                let _ = state.tx_to_client.send(text);
            }

            // Swarm → React
            Ok(msg) = rx.recv() => {
                let _ = socket.send(Message::Text(msg)).await;
            }

            // client disconnected
            else => break,
        }
    }
}