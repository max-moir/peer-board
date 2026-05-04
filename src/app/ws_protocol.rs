use serde::{Deserialize, Serialize};
use crate::core::db::{Message as DbMessage};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsIncoming {
    topic_history { topic: String },
    send_message { topic: String, sender: String, content: String },
    subscribe_topic { topic: String },
    unsubscribe_topic { topic: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsOutgoing {
    history_response {
        topic: String,
        messages: Vec<DbMessage>,
    },
    message {
        topic: String,
        sender: String,
        content: String,
        timestamp: u64,
    },
    error {
        message: String,
    }
}
