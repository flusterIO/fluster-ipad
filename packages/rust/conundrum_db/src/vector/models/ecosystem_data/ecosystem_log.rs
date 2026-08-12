use crate::vector::models::{date_time::date_time::DateTime, primitives::db_id::DatabaseId};
use crate::vector::models::ecosystem_data::ecosystem_log_intention::EcosystemLogIntention;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct EcosystemLog {
    pub id: DatabaseId,
    pub title: String,
    pub message: Option<String>,
    pub purpose: EcosystemLogIntention,
    pub ctime: DateTime
}
