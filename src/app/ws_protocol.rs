use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum WsIncoming {
    message {
        topic: String,
        payload: String,
    },
    history_request {
        topic: String,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum WsOutgoing {
    message {
        topic: String,
        payload: String,
        sender: String,
        timestamp: i64,
    },
    history_response {
        topic: String,
        messages: Vec<String>,
    },
}

// #[derive(Debug, Serialize)]
// pub struct DbMessage {
//     pub message_id: String,
//     pub topic: String,
//     pub sender: String,
//     pub content: String,
//     pub timestamp: u64,
// }