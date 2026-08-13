use crate::vector::models::{
    ai::chat::chat_sender::chat_sender::ChatParticipant, date_time::date_time::DateTime, primitives::db_id::DatabaseId,
};

pub struct ChatMessage {
    pub id: DatabaseId,
    pub conversation_id: DatabaseId,
    pub sender: ChatParticipant,
    pub content: String,
    pub ctime: DateTime,
}
