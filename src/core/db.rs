use rusqlite::{params, Connection, Result};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Message {
    pub message_id: String,
    pub topic: String,
    pub sender: String,
    pub content: String,
    pub timestamp: u64,
}

pub struct MessageStore {
    conn: Connection,
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

        Ok(MessageStore { conn })
    }

    pub fn insert_message(&self, message: &Message) -> Result<()> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM messages WHERE message_id = ?")?;
        let count: i64 = stmt.query_row(params![message.message_id], |row| row.get(0))?;

        if count == 0 {
            self.conn.execute(
                "INSERT INTO messages (message_id, topic, sender, content, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    message.message_id,
                    message.topic,
                    message.sender,
                    message.content,
                    message.timestamp
                ],
            )?;
        }

        Ok(())
    }

    pub fn get_messages_for_topic(&self, topic: &str) -> Result<Vec<Message>> {
        let mut stmt = self.conn.prepare("SELECT message_id, topic, sender, content, timestamp FROM messages WHERE topic = ?")?;
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