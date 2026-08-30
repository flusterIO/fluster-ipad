use std::sync::Arc;

use crate::{
    ai::models::{agent::agent_primary_task::AgentPrimaryTask, tool::mcp_tool_name_list::MCPToolNameList},
    ecosystem::db::{
        db_traits::{db_entity::DBSchema, db_field::DatabaseField},
        parameters::ai::schema_parameters::SchemaParameters,
    },
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_macros::{DBSchema, DatabaseEntity};
use fake::Dummy;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy, DBSchema)]
#[db(source_crate = true)]
pub struct AgentDescriptionPartial {
    pub id: DatabaseId,
    /// The name that the AI should be referred to as. AI should reference this
    /// field when a user asks for another agent by name.
    pub name: Option<String>,
    /// The model to use
    pub model: Option<String>,
    pub reasoning: Option<bool>,
    pub is_local: Option<bool>,
    /// System level instructions
    pub instructions: Option<String>,
    pub always_include_tools: Option<MCPToolNameList>,
    pub temperature_scalar: Option<f32>,
    pub primary_task: Option<AgentPrimaryTask>,
}
