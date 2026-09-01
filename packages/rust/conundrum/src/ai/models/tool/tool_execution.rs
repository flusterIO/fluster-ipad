use std::sync::Arc;

use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use rig::completion::message::ToolCall;
use serde::{Deserialize, Serialize};

use crate::{
    ai::{
        models::tool::mcp_tool_name::MCPToolName,
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
    lifted_models::{
        primitives::{date_time::DateTime, db_id::DatabaseId},
        text::json_content::JsonContent,
    },
};

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::ToolExecution, source_crate = true)]
pub struct ToolExecution {
    pub id: DatabaseId,
    #[db(partial(required))]
    pub tool_name: MCPToolName,
    pub args: Option<JsonContent>,
    pub convo_id: DatabaseId,
    pub agent_id: Option<DatabaseId>,
    pub ctime: DateTime,
}

impl FromWithConvoInformation<<Self as DBSchema>::PartialUpdateType> for ToolExecution {
    fn from_with_convo_info(data: <Self as DBSchema>::PartialUpdateType,
                            convo_id: DatabaseId,
                            agent_id: Option<DatabaseId>)
                            -> Self {
        Self { id: DatabaseId::new(),
               tool_name: data.tool_name,
               convo_id,
               args: data.args.flatten(),
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
                  args: Some(JsonContent::try_from(args)?),
                  agent_id: Some(ctx.agent.clone()),
                  ctime: DateTime::new_now() })
    }
}
