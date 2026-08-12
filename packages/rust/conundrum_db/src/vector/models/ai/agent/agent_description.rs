use std::sync::Arc;

use conundrum::ecosystem::db::{
    tables::DatabaseTable,
    traits::db_entity::{DBEntity, DBSchema},
};
use fake::Dummy;

use crate::{
    impl_default_crud,
    vector::{
        database::db_traits::db_field::DatabaseField,
        models::{
            ai::{
                agent::{agent_description_partial::AgentDescriptionPartial, agent_primary_task::AgentPrimaryTask},
                tool::mcp_tool_name_list::MCPToolNameList,
            },
            date_time::date_time::DateTime,
            primitives::db_id::DatabaseId,
        },
    },
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct AgentDescription {
    pub id: DatabaseId,
    /// The name that the AI should be referred to as. AI should reference this
    /// field when a user asks for another agent by name.
    pub name: Option<String>,
    /// The model to use
    pub model: String,
    /// System level instructions
    pub instructions: Option<String>,
    pub always_include_tools: MCPToolNameList,
    pub temperature: f32,
    pub primary_task: Option<AgentPrimaryTask>,
    pub ctime: DateTime,
    pub utime: DateTime,
}

impl<'a> DBSchema<'a> for AgentDescription {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("name", true)),
                Arc::new(String::field_definition("model", false)),
                Arc::new(String::field_definition("instructions", true)),
                Arc::new(MCPToolNameList::field_definition("always_include_tools", false)),
                Arc::new(f32::field_definition("temperature", false)),
                Arc::new(AgentPrimaryTask::field_definition("primary_task", true)),
                Arc::new(DateTime::field_definition("ctime", false)),
                Arc::new(DateTime::field_definition("utime", false)),])
    }
}

impl<'a> DBEntity<'a, DatabaseId> for AgentDescription {
    type PartialUpdateType = AgentDescriptionPartial;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
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
