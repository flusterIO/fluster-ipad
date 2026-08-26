use std::sync::Arc;

use fake::Dummy;
use rig::completion::message::ToolCall;
use serde::{Deserialize, Serialize};

use crate::{
    ai::{
        models::tool::{mcp_tool_name::MCPToolName, tool_execution_partial::ToolExecutionPartial},
        rig::{
            ai_traits::from_with_convo_information::{FromWithConvoInformation, TryFromWithConvoContext},
            features::chat::convo_context::{ArcMutexConversationContext, ConversationContext},
        },
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

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct ToolExecution {
    pub id: DatabaseId,
    pub tool_name: MCPToolName,
    pub args: serde_json::Value,
    pub convo_id: DatabaseId,
    pub agent_id: Option<DatabaseId>,
    pub ctime: DateTime,
}

impl FromWithConvoInformation<ToolExecutionPartial> for ToolExecution {
    fn from_with_convo_info(data: ToolExecutionPartial, convo_id: DatabaseId, agent_id: Option<DatabaseId>) -> Self {
        Self { id: DatabaseId::new(),
               tool_name: data.tool_name,
               convo_id,
               args: data.args,
               agent_id,
               ctime: DateTime::new_now() }
    }
}

impl TryFromWithConvoContext<ToolCall> for ToolExecution {
    async fn try_from_with_convo_info(data: ToolCall, _ctx: &ArcMutexConversationContext) -> DatabaseResult<Self> {
        let tool_name = MCPToolName::try_from(data.function.name)?;
        let args = data.function.arguments;
        let ctx = _ctx.clone().lock_owned().await;
        Ok(Self { id: DatabaseId::new(),
                  tool_name,
                  convo_id: ctx.convo.clone(),
                  args,
                  agent_id: Some(ctx.agent.clone()),
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

    fn set_primary_value(&mut self, value: DatabaseId) {
        self.id = value.clone();
    }
}

impl_default_crud!(ToolExecution, ToolExecution, DatabaseId);
