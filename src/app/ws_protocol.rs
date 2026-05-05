use serde::{Deserialize, Serialize};
use crate::core::db::{Message as DbMessage};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsIncoming {
    history {},
    send_message { topic: String, nickname: String, content: String },
    subscribe_topic { topic: String },
    unsubscribe_topic { topic: String },
    local_id_req {},

    register_for_game { nickname: String },
    unregister_for_game {},

    send_challenge { peer_id: String, nickname: String },
    respond_challenge { peer_id: String, accepted: bool },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsOutgoing {
    message {
        peer_id: String,
        topic: String,
        content: String,
        timestamp: i64,
        message_id: String,
        nickname: String,
    },
    local_id {
        id: String,
    },
    history_response {
        messages: Vec<DbMessage>,
    },
    error {
        message: String,
    }
}