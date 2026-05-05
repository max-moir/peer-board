use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub message_id: String,
    pub topic: String,
    pub sender: String,
    pub content: String,
    pub timestamp: u64,
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
                topic TEXT NOT NULL,
                sender TEXT NOT NULL,
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
            "INSERT OR IGNORE INTO messages (message_id, topic, sender, content, timestamp)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                message.message_id,
                message.topic,
                message.sender,
                message.content,
                message.timestamp
            ],
        )?;

        Ok(())
    }

    pub fn get_messages_for_topic(&self, topic: &str) -> Result<Vec<Message>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT message_id, topic, sender, content, timestamp
            FROM messages
            WHERE topic = ?
            ORDER BY timestamp ASC"
        )?;

        let message_iter = stmt.query_map(params![topic], |row| {
            Ok(Message {
                message_id: row.get(0)?,
                topic: row.get(1)?,
                sender: row.get(2)?,
                content: row.get(3)?,
                timestamp: row.get(4)?,
            })
        })?;

        let mut messages = Vec::new();
        for message in message_iter {
            messages.push(message?);
        }

        Ok(messages)
    }
}

pub fn current_timestamp() -> u64 {
    let start = SystemTime::now();
    let duration = start.duration_since(UNIX_EPOCH).unwrap();
    duration.as_secs()
}