use std::sync::Arc;

use crate::ecosystem::db::db_traits::db_entity::DBSchema;
use crate::ecosystem::db::db_traits::db_field::DatabaseField;
use crate::lifted_models::primitives::date_time::DateTime;
use crate::lifted_models::primitives::db_id::DatabaseId;
use crate::{
    ai::models::chat::chat_conversation::chat_conversation_partial::ChatConversationPartial,
    ecosystem::db::parameters::ai::schema_parameters::SchemaParameters,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct ChatConversationUtimePartial {
    pub id: DatabaseId,
    pub label: Option<String>,
    pub desc: Option<String>,
    pub requires_label_update: Option<bool>,
    pub utime: DateTime,
}

impl From<ChatConversationPartial> for ChatConversationUtimePartial {
    fn from(value: ChatConversationPartial) -> Self {
        Self { id: value.id.clone(),
               label: value.label.clone(),
               desc: value.desc.clone(),
               requires_label_update: value.requires_label_update.clone(),
               utime: DateTime::new_now() }
    }
}
