use std::sync::Arc;

use fake::Dummy;
use rig::completion::message::ToolCall;
use serde::{Deserialize, Serialize};

use crate::{
    ai::{
        models::tool::mcp_tool_name::MCPToolName,
        rig::ai_traits::from_with_convo_information::TryFromWithConvoInformation,
    },
    ecosystem::{
        db::{
            db_traits::{
                db_entity::{DBEntity, DBSchema},
                db_field::DatabaseField,
            },
            tables::DatabaseTable,
        },
        error_handling::db_error::DatabaseResult,
    },
    impl_default_crud,
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct ToolExecution {
    pub id: DatabaseId,
    pub tool_name: MCPToolName,
    pub args: serde_json::Value,
    pub convo_id: DatabaseId,
    pub agent_id: Option<DatabaseId>,
    pub ctime: DateTime,
}

impl TryFromWithConvoInformation<ToolCall> for ToolExecution {
    fn try_from_with_convo_info(data: ToolCall,
                                convo_id: DatabaseId,
                                agent_id: Option<DatabaseId>)
                                -> DatabaseResult<Self> {
        let tool_name = MCPToolName::try_from(data.function.name)?;
        let args = data.function.arguments;
        Ok(Self { id: DatabaseId::new(),
                  tool_name,
                  convo_id,
                  args,
                  agent_id,
                  ctime: DateTime::new_now() })
    }
}

impl<'a> DBSchema<'a> for ToolExecution {
    fn arrow_fields(
        )
        -> crate::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(DatabaseId::field_definition("tool_name", false)),
                Arc::new(String::field_definition("args", false)),
                Arc::new(DatabaseId::field_definition("convo_id", false)),
                Arc::new(DatabaseId::field_definition("agent_id", false)),
                Arc::new(DateTime::field_definition("ctime", false)),])
    }
}

impl<'a> DBEntity<'a, DatabaseId> for ToolExecution {
    type PartialUpdateType = ToolExecution;

    fn table() -> crate::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::ToolExecution
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

impl_default_crud!(ToolExecution, ToolExecution, DatabaseId);
