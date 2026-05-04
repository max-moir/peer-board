use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use chrono::{Utc};
use prost::Message;
use uuid::Uuid;

use crate::core::proto::peerboard::v1::PeerBoardMessage;

#[derive(Clone, Default)]
pub struct MessageDedup {
    inner: Arc<Mutex<HashSet<String>>>,
}

impl MessageDedup {
    pub fn seen(&self, id: &str) -> bool {
        let mut set = self.inner.lock().unwrap();
        if set.contains(id) {
            true
        } else {
            set.insert(id.to_string());
            false
        }
    }
}

pub fn encode_message(
    peer_id: &str,
    topic: &str,
    content: String,
    nickname: String,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    if content.as_bytes().len() > 4096 {
        return Err("content too large".into());
    }

    if nickname.as_bytes().len() > 32 {
        return Err("nickname too large".into());
    }

    if !topic.starts_with("peerboard/v1/") {
        return Err("invalid topic prefix".into());
    }

    let msg = PeerBoardMessage {
        peer_id: peer_id.to_string(),
        topic: topic.to_string(),
        content,
        timestamp: Utc::now().timestamp(),
        message_id: Uuid::new_v4().to_string(),
        nickname,
    };

    let mut buf = Vec::new();
    msg.encode(&mut buf)?;
    Ok(buf)
}


pub fn decode_and_validate_message(
    bytes: &[u8],
    dedup: &MessageDedup,
) -> Option<PeerBoardMessage> {

    let msg = match PeerBoardMessage::decode(bytes) {
        Ok(m) => m,
        Err(_) => return None, // silent drop
    };

    if dedup.seen(&msg.message_id) {
        return None;
    }

    if !msg.topic.starts_with("peerboard/v1/") {
        return None;
    }

    if msg.content.as_bytes().len() > 4096 {
        return None;
    }

    if msg.nickname.as_bytes().len() > 32 {
        return None;
    }

    let now = Utc::now().timestamp();
    if msg.timestamp > now + 300 {
        return None;
    }

    Some(msg)
}