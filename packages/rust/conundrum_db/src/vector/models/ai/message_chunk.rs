use conundrum::{
    ai::models::chat::chat_message::chat_message_sender::ChatMessageSender,
    ecosystem::db::{db_traits::db_entity::DBEntity, tables::DatabaseTable},
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::vector::vector::DBVector;

#[derive(Serialize, Deserialize, Clone, Debug, DatabaseEntity, specta::Type, Dummy)]
#[db(table = DatabaseTable::MessageChunk)]
pub struct MessageChunk {
    pub id: DatabaseId,
    pub msg_id: DatabaseId,
    pub convo_id: DatabaseId,
    pub text: String,
    pub sender: ChatMessageSender,
    pub remote_vector: DBVector,
    pub local_vector: DBVector,
}
