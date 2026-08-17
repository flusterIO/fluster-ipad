use conundrum::lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId};

use crate::vector::models::lifestyle::fitness::models::biological_gender::BiologicalGender;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct UserPhysicalStats {
    pub id: DatabaseId,
    pub date_of_birth: Option<DateTime>,
    /// The user's height in centimeters.
    pub height: Option<f32>,
    /// The user's weight in kilograms.
    pub weight: Option<f32>,
    pub biological_gender: Option<BiologicalGender>,
    /// The user participates in these sports, and might like advice training
    /// for these specific goals.
    pub sports: Vec<String>,
    /// AI, these notes are for you. If the user has some unique physical goals
    /// or strenghts, like an exceptional bench-press or an amazing vertical
    /// leap, they may mention that here. Take into account these notes when
    /// creating all exercise related suggestions, as they come directly
    /// from the user.
    pub notes: Option<String>,
}
