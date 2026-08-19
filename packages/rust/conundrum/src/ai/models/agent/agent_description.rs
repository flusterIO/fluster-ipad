use fake::Dummy;
use std::sync::Arc;

use crate::{
    ai::models::{
        agent::{agent_description_partial::AgentDescriptionPartial, agent_primary_task::AgentPrimaryTask},
        tool::{mcp_tool_name::MCPToolName, mcp_tool_name_list::MCPToolNameList},
    },
    ecosystem::db::{
        db_traits::{
            db_entity::{DBEntity, DBSchema},
            db_field::DatabaseField,
        },
        tables::DatabaseTable,
    },
    impl_default_crud,
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct AgentDescription {
    pub id: DatabaseId,
    /// The name that the AI should be referred to as. AI should reference this
    /// field when a user asks for another agent by name.
    pub name: Option<String>,
    pub max_tokens: Option<u32>,
    pub allow_tools: bool,
    /// The model to use
    pub model: String,
    pub reasoning: bool,
    pub is_local: bool,
    /// System level instructions
    pub instructions: Option<String>,
    pub always_include_tools: MCPToolNameList,
    /// A scalar that will be applied to the temperature assigned to each task.
    /// Defaults to 1, the same as being null.
    pub temperature_scalar: f32,
    pub primary_task: Option<AgentPrimaryTask>,
    pub ctime: DateTime,
    pub utime: DateTime,
}

impl Default for AgentDescription {
    fn default() -> Self {
        Self { 
            id: DatabaseId::new(),
            name: None,
            max_tokens: Some(1024),
            allow_tools: true,
            model: "qwen3:8b".to_string(),
            reasoning: true,
            is_local: true,
            instructions: None,
            always_include_tools: MCPToolNameList::new_empty(),
            temperature_scalar: 1.,
            primary_task: Some(AgentPrimaryTask::Agent),
            ctime: DateTime::new_now(),
            utime: DateTime::new_now()
        }
    }
}

impl AgentDescription {
    pub fn default_local_chat() -> Self {
        AgentDescription {
            id: DatabaseId::default(),
            name: None,
            model: "qwen3:8b".to_string(),
            max_tokens: Some(1024),
            allow_tools: true, 
            reasoning: true,
            is_local: true,
            instructions: Some("You are an assistant for an academic research platform for STEM students and professionals.".to_string()),
            always_include_tools: MCPToolNameList::new_empty(),
            temperature_scalar: 1.,
            primary_task: None,
            ctime: DateTime::new_now(),
            utime: DateTime::new_now()
        }
    }

    /// Only to be used for local development.
    pub fn local_debug_llm() -> Self {
        Self::default_local_chat()
    }
}

impl<'a> DBSchema<'a> for AgentDescription {
    fn arrow_fields(
        )
        -> crate::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("name", true)),
                Arc::new(String::field_definition("model", false)),
                Arc::new(bool::field_definition("reasoning", false)),
                Arc::new(bool::field_definition("is_local", false)),
                Arc::new(String::field_definition("instructions", true)),
                Arc::new(MCPToolNameList::field_definition("always_include_tools", false)),
                Arc::new(f32::field_definition("temperature_scalar", false)),
                Arc::new(AgentPrimaryTask::field_definition("primary_task", true)),
                Arc::new(DateTime::field_definition("ctime", false)),
                Arc::new(DateTime::field_definition("utime", false)),])
    }
}

impl<'a> DBEntity<'a, DatabaseId> for AgentDescription {
    type PartialUpdateType = AgentDescriptionPartial;

    fn table() -> crate::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::AgentDescription
    }

    fn merge_keys() -> &'static [&'static str] {
        &["id"]
    }

    fn primary_key() -> &'static str {
        "id"
    }

    fn primary_value(&self) -> DatabaseId {
        self.id.clone()
    }
}

impl_default_crud!(AgentDescription, AgentDescriptionPartial, DatabaseId);
