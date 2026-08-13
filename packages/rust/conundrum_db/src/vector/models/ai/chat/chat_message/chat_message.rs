use crate::vector::models::{ai::chat::chat_sender::chat_sender::ChatParticipant, date_time::date_time::DateTime};

pub struct ChatMessage {
    pub sender: ChatParticipant,
    pub content: String,
    pub ctime: DateTime,
}
