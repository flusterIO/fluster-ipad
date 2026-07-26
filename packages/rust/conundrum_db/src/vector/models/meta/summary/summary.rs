use crate::vector::models::date_time::date_time::DateTime;

/// ## ModelSummary
///
/// A general 'summary' that can be passed to AI. Obviously this is most useful
/// for text based Conundrum notes, but this can be used in a much wider
/// application.
pub struct SummaryModel {
    pub body: String,
    pub ai_generated: bool,
    pub ctime: DateTime,
}
