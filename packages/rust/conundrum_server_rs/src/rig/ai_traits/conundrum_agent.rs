use conundrum_db::vector::models::ai::chat::chat_message::chat_message::ChatMessage;
use rig::Agent;
use rig::completion::CompletionModel;

pub trait ConundrumAgent<CompletionModelType: CompletionModel + Sized> {
    fn stream_chat_response(&self, agent: Agent<CompletionModelType>, chat_request: ChatMessage);
}
