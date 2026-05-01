use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct AppState {
    pub tx: mpsc::UnboundedSender<String>,
}

#[derive(Deserialize)]
pub struct SendMessage {
    pub content: String,
    pub nickname: String,
}

pub async fn run_server(state: AppState) {

    let app = Router::new()
        .route("/health", get(health))
        .route("/send", post(send_message))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}

async fn health() -> &'static str {
    "ok"
}

async fn send_message(
    State(state): State<AppState>,
    Json(payload): Json<SendMessage>,
) -> &'static str {

    let msg = format!("{}:{}", payload.nickname, payload.content);

    let _ = state.tx.send(msg);

    "sent"
}