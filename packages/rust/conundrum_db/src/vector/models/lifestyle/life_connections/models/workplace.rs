use crate::vector::models::{
    ai::ai_interactions::AIInteractions,
    lifestyle::life_connections::models::physical_street_address::PhysicalStreetAddress,
};

// # Workplace
// Describes the workplace of the acquaintances in the user's database.
pub struct Workplace {
    pub company_name: Option<String>,
    pub location: PhysicalStreetAddress,
    pub ai: AIInteractions,
}
