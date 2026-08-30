use std::sync::Arc;

use crate::vector::models::ecosystem_data::log::ecosystem_log_input::EcosystemLogInput;
use crate::vector::models::ecosystem_data::log::ecosystem_log_intention::EcosystemLogIntention;
use crate::vector::models::ecosystem_data::log::ecosystem_log_severity::EcosystemLogSeverity;
use conundrum::ecosystem::db::db_traits::db_entity::{DBEntity, DBSchema};
use conundrum::ecosystem::db::db_traits::db_field::DatabaseField;
use conundrum::ecosystem::db::tables::DatabaseTable;
use conundrum::impl_default_crud;
use conundrum::lifted_models::primitives::date_time::DateTime;
use conundrum::lifted_models::primitives::db_id::DatabaseId;
use conundrum_macros::DatabaseEntity;
use fake::Dummy;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::EcosystemLog)]
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
