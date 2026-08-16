use std::sync::Arc;

use crate::impl_default_crud;
use crate::vector::database::db_traits::db_field::DatabaseField;
use crate::vector::models::ecosystem_data::log::ecosystem_log_input::EcosystemLogInput;
use crate::vector::models::ecosystem_data::log::ecosystem_log_intention::EcosystemLogIntention;
use crate::vector::models::ecosystem_data::log::ecosystem_log_severity::EcosystemLogSeverity;
use crate::vector::models::{date_time::date_time::DateTime, primitives::db_id::DatabaseId};
use conundrum::ecosystem::db::tables::DatabaseTable;
use conundrum::ecosystem::db::traits::db_entity::{DBEntity, DBSchema};
use fake::Dummy;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct EcosystemLog {
    pub id: DatabaseId,
    pub title: String,
    /// An optional user facing message.
    pub message: Option<String>,
    /// A description of the event logged written directly to AI.
    pub ai_description: String,
    pub purpose: EcosystemLogIntention,
    pub severity: EcosystemLogSeverity,
    pub ctime: DateTime,
}

impl<'a> DBEntity<'a, DatabaseId> for EcosystemLog {
    type PartialUpdateType = EcosystemLogInput;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::EcosystemLog
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

impl_default_crud!(EcosystemLog, EcosystemLogInput, DatabaseId);

impl<'a> DBSchema<'a> for EcosystemLog {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("title", false)),
                Arc::new(String::field_definition("message", true)),
                Arc::new(String::field_definition("ai_description", false)),
                Arc::new(EcosystemLogIntention::field_definition("purpose", false)),
                Arc::new(EcosystemLogSeverity::field_definition("severity", false)),
                Arc::new(DateTime::field_definition("ctime", false))])
    }
}

impl From<EcosystemLogInput> for EcosystemLog {
    fn from(value: EcosystemLogInput) -> Self {
        Self { id: DatabaseId::default(),
               title: value.title.clone(),
               message: value.message.clone(),
               ai_description: value.ai_description.clone(),
               purpose: value.purpose.clone(),
               severity: value.severity.clone(),
               ctime: DateTime::new_now() }
    }
}
