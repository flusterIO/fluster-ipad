use conundrum::lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId};

use crate::vector::models::{
    ai::ai_interactions::AIInteractions, lifestyle::fitness::models::general_bodypart::BodyPart,
};

/// # PersonalInjury
///
/// This struct documents a user's injury. If this injury is still an 'existing
/// concern', make sure to avoid making exercise related suggestions that might
/// exacerbate this injury further.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct PersonalInjury {
    pub id: DatabaseId,
    pub label: String,
    pub description: Option<String>,
    pub is_existing_concern: bool,
    pub occurred_on: Option<DateTime>,
    /// If this vector is empty, consider this a general injury.
    pub bodyparts: Vec<BodyPart>,
    pub ai: AIInteractions,
}
