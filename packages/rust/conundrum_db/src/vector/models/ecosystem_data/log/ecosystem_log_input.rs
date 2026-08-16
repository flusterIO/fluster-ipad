use std::sync::Arc;

use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use fake::Dummy;

use crate::vector::{
    database::db_traits::db_field::DatabaseField,
    models::ecosystem_data::log::{
        ecosystem_log_intention::EcosystemLogIntention, ecosystem_log_severity::EcosystemLogSeverity,
    },
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct EcosystemLogInput {
    pub title: String,
    pub message: Option<String>,
    pub ai_description: String,
    pub purpose: EcosystemLogIntention,
    pub severity: EcosystemLogSeverity,
}

impl<'a> DBSchema<'a> for EcosystemLogInput {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(String::field_definition("title", false)),
                Arc::new(String::field_definition("message", true)),
                Arc::new(String::field_definition("ai_description", false)),
                Arc::new(EcosystemLogIntention::field_definition("purpose", false)),
                Arc::new(EcosystemLogSeverity::field_definition("severity", false)),])
    }
}
