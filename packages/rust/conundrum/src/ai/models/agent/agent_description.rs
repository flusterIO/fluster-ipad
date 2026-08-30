use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use std::sync::Arc;

use crate::{
    ai::{
        ai_constants::DEFAULT_LOCAL_LANGUAGE_MODEL,
        models::{
            agent::agent_primary_task::AgentPrimaryTask,
            chat::chat_conversation::vector_mode::VectorMode,
            tool::{mcp_tool_name::MCPToolName, mcp_tool_name_list::MCPToolNameList},
        },
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
use indoc::formatdoc;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::AgentDescription, source_crate = true)]
pub struct AgentDescription {
    pub id: DatabaseId,
    /// The name that the AI should be referred to as. AI should reference this
    /// field when a user asks for another agent by name.
    pub name: Option<String>,
    pub max_tokens: Option<u32>,
    pub allow_tools: bool,
    pub embedding_mode: VectorMode,
    pub chat_mode: VectorMode,
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
        Self { id: DatabaseId::new(),
               name: None,
               max_tokens: Some(1024),
               allow_tools: true,
               embedding_mode: VectorMode::Remote,
               chat_mode: VectorMode::Remote,
               model: DEFAULT_LOCAL_LANGUAGE_MODEL.to_string(),
               reasoning: true,
               is_local: true,
               instructions: None,
               always_include_tools: MCPToolNameList::new_empty(),
               temperature_scalar: 1.,
               primary_task: Some(AgentPrimaryTask::Agent),
               ctime: DateTime::new_now(),
               utime: DateTime::new_now() }
    }
}

impl AgentDescription {
    pub fn default_local_chat() -> Self {
        AgentDescription { id: DatabaseId::default(),
                           name: None,
                           model: DEFAULT_LOCAL_LANGUAGE_MODEL.to_string(),
                           max_tokens: Some(1024),
                           allow_tools: true,
                           reasoning: true,
                           is_local: true,
                           embedding_mode: VectorMode::Remote,
                           chat_mode: VectorMode::Remote,
                           instructions: Some(formatdoc! {"
            You are an assistant for an academic note taking application for students and professionals, especially those interested in STEM fields.
            Use the tools available to you to help each student or researcher reach their academic goals.
                "}),
                           always_include_tools: MCPToolNameList::new_empty(),
                           temperature_scalar: 1.,
                           primary_task: None,
                           ctime: DateTime::new_now(),
                           utime: DateTime::new_now() }
    }

    /// Only to be used for local development.
    pub fn local_debug_llm() -> Self {
        Self::default_local_chat()
    }
}
