use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

use crate::vector::models::{date_time::date_time::DateTime, primitives::db_id::DatabaseId};

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub enum SummarySource {
    Conundrum,
    Typst,
    Html,
}

/// ## ModelSummary
///
/// A general 'summary' that can be passed to AI. Obviously this is most useful
/// for text based Conundrum notes, but this can be used in a much wider
/// application.
#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct SummaryModel {
    pub body: String,
    pub ai_generated: bool,
    pub ctime: DateTime,
    pub source_type: SummarySource,
    pub source_id: DatabaseId,
}
