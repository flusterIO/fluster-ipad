use crate::vector::models::{ai::agent::agent_primary_task::AgentPrimaryTask, primitives::db_id::DatabaseId};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct AgentDescription {
    pub id: DatabaseId,
    /// The name that the AI should be referred to as. AI should reference this
    /// field when a user asks for another agent by name.
    pub name: Option<String>,
    /// The model to use
    pub model: String,
    pub instructions: String,
    pub temperature: f32,
    pub primary_task: Option<AgentPrimaryTask>,
}
