pub trait IntoChatHistoryItem {
    #[allow(clippy::wrong_self_convention)]
    fn into_chat_message_history_item(&self) -> String;
}
