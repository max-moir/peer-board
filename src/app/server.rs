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
use crate::app::ws_protocol::{WsIncoming, WsOutgoing};


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
                let parsed = serde_json::from_str::<WsIncoming>(&text);

                // Incoming websocket events
                match parsed {
                    Ok(WsIncoming::history {}) => {
                        match state.db.get_all_messages() {
                            Ok(messages) => {

                                let _ = state.tx_to_swarm.send(WsIncoming::local_id_req{}).await;
                                let outgoing = WsOutgoing::history_response {
                                    messages,
                                };

                                if let Ok(json) = serde_json::to_string(&outgoing) {
                                    let _ = socket.send(Message::Text(json)).await;
                                }
                            }

                            Err(e) => {
                                let err = WsOutgoing::error {
                                    message: format!("DB error: {e}"),
                                };

                                let _ = socket.send(
                                    Message::Text(serde_json::to_string(&err).unwrap())
                                ).await;
                            }
                        }

                    },

                    // Commands for swarm_runner
                    Ok(incoming_command) => {
                        let _ = state.tx_to_swarm.send(incoming_command).await;
                    },

                    Err(e) => {
                        eprintln!("WS parse error: {:?}", e);
                        continue;
                    }
                }
            }

            Ok(msg) = rx.recv() => {
                match msg {
                    WsOutgoing::message { .. } => {
                        if let Ok(json) = serde_json::to_string(&msg) {
                            let _ = socket.send(Message::Text(json)).await;
                        }
                    },
                    WsOutgoing::local_id { .. } => {
                        if let Ok(json) = serde_json::to_string(&msg) {
                            let _ = socket.send(Message::Text(json)).await;
                        }
                    }
                    _ => {}
                }
            }

            else => break,
        }
    }
}