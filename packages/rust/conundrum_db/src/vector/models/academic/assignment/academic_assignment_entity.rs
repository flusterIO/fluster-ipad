use std::sync::Arc;

use conundrum::{
    ecosystem::db::{
        db_traits::{
            db_entity::{DBEntity, DBSchema},
            db_field::DatabaseField,
        },
        tables::DatabaseTable,
    },
    impl_default_crud,
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::academic::assignment::academic_assignment_entity_partial::AssignmentEntityPartial;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct AssignmentEntity {
    pub id: DatabaseId,
    pub label: String,
    pub description: Option<String>,
    pub due_at: Option<DateTime>,
    pub ctime: DateTime,
    pub utime: DateTime,
}

impl<'a> DBSchema<'a> for AssignmentEntity {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("label", false)),
                Arc::new(String::field_definition("description", true)),
                Arc::new(DateTime::field_definition("due_at", true)),
                Arc::new(DateTime::field_definition("ctime", false)),
                Arc::new(DateTime::field_definition("utime", false)),])
    }
}

impl<'a> DBEntity<'a, DatabaseId> for AssignmentEntity {
    type PartialUpdateType = AssignmentEntityPartial;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::Assignment
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

impl_default_crud!(AssignmentEntity, AssignmentEntityPartial, DatabaseId);
