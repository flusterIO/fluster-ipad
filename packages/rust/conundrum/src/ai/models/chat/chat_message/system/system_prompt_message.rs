use std::sync::Arc;

use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::{
    ai::rig::ai_traits::from_with_convo_information::FromWithConvoInformation,
    ecosystem::db::{
        db_traits::{
            db_entity::{DBEntity, DBSchema},
            db_field::DatabaseField,
        },
        macros::impl_default_crud,
        tables::DatabaseTable,
    },
    impl_default_crud,
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct SystemPromptMessage {
    pub id: DatabaseId,
    pub body: String,
    pub convo_id: DatabaseId,
    pub ctime: DateTime,
}

impl FromWithConvoInformation<String> for SystemPromptMessage {
    fn from_with_convo_info(data: String, convo_id: DatabaseId, agent_id: Option<DatabaseId>) -> Self {
        Self { id: DatabaseId::new(),
               body: data,
               convo_id,
               ctime: DateTime::new_now() }
    }
}

impl<'a> DBSchema<'a> for SystemPromptMessage {
    fn arrow_fields(
        )
        -> crate::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("body", false)),
                Arc::new(DatabaseId::field_definition("convo_id", false)),
                Arc::new(DateTime::field_definition("ctime", false)),])
    }
}

impl<'a> DBEntity<'a, DatabaseId> for SystemPromptMessage {
    type PartialUpdateType = SystemPromptMessage;

    fn table() -> crate::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::SystemPromptMessage
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

impl_default_crud!(SystemPromptMessage, SystemPromptMessage, DatabaseId);
