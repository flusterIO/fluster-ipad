use std::sync::Arc;

use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use fake::Dummy;

use crate::vector::{
    database::db_traits::db_field::DatabaseField,
    models::{
        ai::{agent::agent_primary_task::AgentPrimaryTask, tool::mcp_tool_name_list::MCPToolNameList},
        primitives::db_id::DatabaseId,
    },
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
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

impl<'a> DBSchema<'a> for AgentDescriptionPartial {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("name", true)),
                Arc::new(String::field_definition("model", true)),
                Arc::new(bool::field_definition("reasoning", true)),
                Arc::new(bool::field_definition("is_local", true)),
                Arc::new(String::field_definition("instructions", true)),
                Arc::new(MCPToolNameList::field_definition("always_include_tools", true)),
                Arc::new(f32::field_definition("temperature_scalar", true)),
                Arc::new(AgentPrimaryTask::field_definition("primary_task", true)),])
    }
}
