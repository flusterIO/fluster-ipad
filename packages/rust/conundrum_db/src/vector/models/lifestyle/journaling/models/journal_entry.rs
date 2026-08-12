use crate::vector::models::{
    lifestyle::journaling::models::journal_entry_sentiment::JournalEntrySentiment, primitives::db_id::DatabaseId,
};

pub struct JournalEntry {
    /// The id of the conundrum note in the database.
    pub note_id: DatabaseId,
    /// The general sentiment of the journal entry. This field is **not**
    /// optional for AI. AI should always perform sentiment analysis if the
    /// available tools and accessible data allow for it.
    pub sentiment: Option<JournalEntrySentiment>,
}
