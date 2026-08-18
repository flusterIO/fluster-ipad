use std::sync::Arc;

use crate::ecosystem::db::db_traits::db_field::DatabaseField;
use crate::ecosystem::db::{db_traits::db_entity::DBSchema, parameters::ai::schema_parameters::SchemaParameters};
use crate::lifted_models::primitives::db_id::DatabaseId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct ChatConversationPartial {
    pub id: DatabaseId,
    pub label: Option<String>,
    pub requires_label_update: Option<bool>,
    pub desc: Option<String>,
}

impl<'a> DBSchema<'a> for ChatConversationPartial {
    fn arrow_fields(
        )
        -> crate::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("label", true)),
                Arc::new(bool::field_definition("requires_label_update", true)),
                Arc::new(String::field_definition("desc", true)),])
    }
}
