use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub message_id: String,
    pub peer_id: String,       
    pub topic: String,         
    pub nickname: String,      
    pub content: String,
    pub timestamp: i64,        
}

pub struct MessageStore {
    conn: std::sync::Mutex<Connection>,
}

impl MessageStore {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                message_id TEXT PRIMARY KEY,
                peer_id TEXT NOT NULL,
                topic TEXT NOT NULL,
                nickname TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(MessageStore {
            conn: std::sync::Mutex::new(conn),
        })
    }

    pub fn insert_message(&self, message: &Message) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "INSERT OR IGNORE INTO messages (message_id, peer_id, topic, nickname, content, timestamp)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                message.message_id,
                message.peer_id,
                message.topic,
                message.nickname,
                message.content,
                message.timestamp
            ],
        )?;

        Ok(())
    }

    pub fn get_all_messages(&self) -> Result<Vec<Message>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT message_id, peer_id, topic, nickname, content, timestamp
            FROM messages
            ORDER BY timestamp ASC"
        )?;

        let message_iter = stmt.query_map([], |row| {
            Ok(Message {
                message_id: row.get(0)?,
                peer_id: row.get(1)?,
                topic: row.get(2)?,
                nickname: row.get(3)?,
                content: row.get(4)?,
                timestamp: row.get(5)?,
            })
        })?;

        let mut messages = Vec::new();
        for message in message_iter {
            messages.push(message?);
        }

        Ok(messages)
    }
}

pub fn current_timestamp() -> i64 {
    let start = SystemTime::now();
    let duration = start.duration_since(UNIX_EPOCH).unwrap();
    duration.as_secs() as i64
}