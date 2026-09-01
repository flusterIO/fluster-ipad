use std::sync::Arc;

use conundrum::ecosystem::db::db_traits::{db_entity::DBSchema, db_field::DatabaseField};
use conundrum_macros::DBSchema;
use fake::Dummy;

use crate::vector::models::ecosystem_data::log::{
    ecosystem_log_intention::EcosystemLogIntention, ecosystem_log_severity::EcosystemLogSeverity,
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct EcosystemLogInput {
    pub title: String,
    pub message: Option<String>,
    pub ai_description: String,
    pub purpose: EcosystemLogIntention,
    pub severity: EcosystemLogSeverity,
}
