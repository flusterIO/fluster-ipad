use std::sync::Arc;

use crate::ecosystem::db::db_traits::db_entity::{DBEntity, DBSchema};
use crate::ecosystem::db::parameters::ai::schema_parameters::SchemaParameters;
use crate::ecosystem::db::tables::DatabaseTable;
use crate::impl_default_crud;
use crate::lifted_models::primitives::date_time::DateTime;
use crate::lifted_models::primitives::db_id::DatabaseId;
use crate::{
    ai::models::chat::chat_conversation::chat_conversation_partial::ChatConversationPartial,
    ecosystem::db::db_traits::db_field::DatabaseField,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct ChatConversation {
    pub id: DatabaseId,
    pub label: String,
    pub desc: Option<String>,
    pub requires_label_update: bool,
    pub ctime: DateTime,
    #[serde(default = "DateTime::new_now")]
    pub utime: DateTime,
}

impl_default_crud!(ChatConversation, ChatConversationPartial, DatabaseId);

impl<'a> DBSchema<'a> for ChatConversation {
    fn arrow_fields(
        )
        -> crate::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("label", false)),
                Arc::new(String::field_definition("desc", true)),
                Arc::new(bool::field_definition("requires_label_update", true)),
                Arc::new(DateTime::field_definition("ctime", false)),
                Arc::new(DateTime::field_definition("utime", false))])
    }
}

impl<'a> DBEntity<'a, DatabaseId> for ChatConversation {
    type PartialUpdateType = ChatConversation;

    fn table() -> crate::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::ChatConversation
    }

    fn merge_keys() -> &'static [&'static str] {
        &["id"]
    }

    fn primary_key() -> &'static str {
        "id"
    }

    fn primary_value(&self) -> DatabaseId {
        self.id.clone()
    }
}
