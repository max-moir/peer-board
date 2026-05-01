use chrono::Utc;
use prost::Message;
use uuid::Uuid;

use crate::core::proto::peerboard::v1::PeerBoardMessage;

pub fn encode_message(
    peer_id: &str,
    topic: &str,
    content: String,
    nickname: String,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    if content.len() > 4096 {
        return Err("content too large".into());
    }

    if nickname.len() > 32 {
        return Err("nickname too large".into());
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

pub fn decode_message(
    bytes: &[u8],
) -> Result<PeerBoardMessage, prost::DecodeError> {
    PeerBoardMessage::decode(bytes)
}