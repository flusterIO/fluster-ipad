use crate::vector::models::{
    ai::ai_generated_input::AIGeneratedInput,
    lifestyle::life_connections::models::geographic_location::GeographicLocation,
};

pub struct PlaceOfSignificance {
    /// The significance of this place to the user.
    pub significance: Option<String>,
    pub location: Option<GeographicLocation>,
    pub ai: AIGeneratedInput,
}
