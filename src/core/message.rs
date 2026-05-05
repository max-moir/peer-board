use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use base64::{encode, decode};

use chrono::Utc;
use prost::Message;
use uuid::Uuid;

use crate::core::proto::peerboard::v1::{
    PeerBoardMessage,
};
use crate::core::proto::peerboard::challenge::v1::{
    ChallengePropose,
    ChallengeResponse,
    BattleshipRequest,
    BattleshipResponse,
};

const PREFIX: &str = "peerboard/v1/";

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

/// Encode a generic PeerBoardMessage (chat or game)
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
    if !topic.starts_with(PREFIX) {
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

/// Decode and validate a PeerBoardMessage (chat or game)
pub fn decode_and_validate_message(
    bytes: &[u8],
    dedup: &MessageDedup,
) -> Option<PeerBoardMessage> {
    let mut msg = match PeerBoardMessage::decode(bytes) {
        Ok(m) => m,
        Err(_) => return None,
    };

    if dedup.seen(&msg.message_id) {
        return None;
    }
    if !msg.topic.starts_with(PREFIX) {
        return None;
    }
    if msg.content.as_bytes().len() > 4096 {
        return None;
    }

    msg.topic = msg.topic.strip_prefix(PREFIX).unwrap_or(&msg.topic).to_string();

    if msg.nickname.as_bytes().len() > 32 {
        return None;
    }

    let now = Utc::now().timestamp();
    if msg.timestamp > now + 300 {
        return None;
    }

    Some(msg)
}

/// ---------------------------------------------
/// Battleship helper functions
/// ---------------------------------------------

/// Encode a BattleshipRequest into a PeerBoardMessage
pub fn encode_battleship_request(
    peer_id: &str,
    topic: &str,
    request: BattleshipRequest,
    nickname: String,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    request.encode(&mut buf)?;
    encode_message(peer_id, topic, base64::encode(buf), nickname)
}

/// Decode a BattleshipRequest from a PeerBoardMessage
pub fn decode_battleship_request(
    msg: &PeerBoardMessage,
) -> Option<BattleshipRequest> {
    let data = base64::decode(&msg.content).ok()?;
    BattleshipRequest::decode(&*data).ok()
}

/// Encode a BattleshipResponse into a PeerBoardMessage
pub fn encode_battleship_response(
    peer_id: &str,
    topic: &str,
    response: BattleshipResponse,
    nickname: String,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    response.encode(&mut buf)?;
    encode_message(peer_id, topic, base64::encode(buf), nickname)
}

/// Decode a BattleshipResponse from a PeerBoardMessage
pub fn decode_battleship_response(
    msg: &PeerBoardMessage,
) -> Option<BattleshipResponse> {
    let data = base64::decode(&msg.content).ok()?;
    BattleshipResponse::decode(&*data).ok()
}