use std::sync::Arc;

use crate::{
    impl_default_crud,
    vector::{
        database::db_traits::db_field::DatabaseField,
        models::{
            ai::chat::chat_conversation::chat_conversation_partial::ChatConversationPartial,
            date_time::date_time::DateTime,
            primitives::{db_id::DatabaseId, helper_models::label_and_id::IDAndOptionalLabel},
        },
    },
};
use conundrum::ecosystem::db::{
    tables::DatabaseTable,
    traits::db_entity::{DBEntity, DBSchema},
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
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
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

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
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
