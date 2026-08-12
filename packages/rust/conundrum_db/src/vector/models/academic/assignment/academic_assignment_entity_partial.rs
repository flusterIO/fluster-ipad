use std::sync::Arc;

use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_traits::db_field::DatabaseField,
    models::{date_time::date_time::DateTime, primitives::db_id::DatabaseId},
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct AssignmentEntityPartial {
    pub id: DatabaseId,
    pub label: Option<String>,
    pub description: Option<String>,
    pub due_at: Option<DateTime>,
}

impl<'a> DBSchema<'a> for AssignmentEntityPartial {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("label", true)),
                Arc::new(String::field_definition("description", true)),
                Arc::new(DateTime::field_definition("due_at", true)),])
    }
}
