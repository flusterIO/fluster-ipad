use conundrum::lifted_models::primitives::date_time::DateTime;

use crate::vector::models::{
    ai::ai_interactions::AIInteractions,
    lifestyle::life_connections::models::{geographic_location::GeographicLocation, participants::Participants},
};

/// # MajorLifeEvent
///
/// Describe a user's life as you get to know them. If they graduated school,
/// congratulate them and save the event here. If they mention the hopeful birth
/// of a child, or a trip that they're excited for, create a `MajorLifeEvent` so
/// you can help them prepare.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct MajorLifeEvent {
    pub label: String,
    pub description: Option<String>,
    /// The time at which the event happened, or is expected to happen.
    pub date: Option<DateTime>,
    pub ai: AIInteractions,
    /// The location the event occurred at or is expected to occur.
    pub location: Option<GeographicLocation>,
    #[serde(default)]
    pub participants: Participants,
}
