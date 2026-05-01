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

    println!("WS running on ws://127.0.0.1:3001/ws");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
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

            Some(Ok(Message::Text(text))) = socket.recv() => {
                let _ = state.tx_to_swarm.send(text).await;
            }

            Ok(msg) = rx.recv() => {
                let _ = socket.send(Message::Text(msg)).await;
            }

            else => break,
        }
    }
}