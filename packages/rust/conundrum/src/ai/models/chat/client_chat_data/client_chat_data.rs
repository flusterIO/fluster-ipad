use crate::{
    ai::{
        models::{
            chat::{
                chat_conversation::token_expendeture::TokenExpendeture,
                chat_message::{
                    ai::reasoning_block::ReasoningBlock, system::system_prompt_message::SystemPromptMessage,
                    user::user_message::UserMessage,
                },
            },
            tool::{
                tool_execution::{self, ToolExecution},
                tool_execution_partial::ToolExecutionPartial,
            },
        },
        rig::ai_traits::from_with_convo_information::FromWithConvoInformation,
    },
    ecosystem::db::db_traits::db_entity::DBEntity,
    lifted_models::primitives::db_id::DatabaseId,
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct ClientChatData {
    pub convo_id: DatabaseId,
    pub agent_id: Option<DatabaseId>,
    pub reasoning: Vec<String>,
    pub system_prompt: Option<String>,
    pub response: String,
    pub tool_calls: Vec<<ToolExecution as DBEntity>::PartialUpdateType>,
    pub tokens: TokenExpendeture,
}

impl ClientChatData {
    pub fn expand(&self) -> (UserMessage, Vec<ReasoningBlock>, Option<SystemPromptMessage>, Vec<ToolExecution>) {
        let user_message =
            UserMessage::from_with_convo_info(self.response.clone(), self.convo_id.clone(), self.agent_id.clone());
        let reasoning_blocks =
            self.reasoning
                .iter()
                .map(|x| ReasoningBlock::from_with_convo_info(x.clone(), self.convo_id.clone(), self.agent_id.clone()))
                .collect::<Vec<ReasoningBlock>>();
        let system_prompt =
            self.system_prompt.clone().map(|x| {
                                          SystemPromptMessage::from_with_convo_info(x,
                                                                                    self.convo_id.clone(),
                                                                                    self.agent_id.clone())
                                      });
        let tool_executions: Vec<ToolExecution> =
            self.tool_calls
                .iter()
                .map(|x: &<ToolExecution as DBEntity>::PartialUpdateType| {
                    ToolExecution::from_with_convo_info(x.clone(), self.convo_id.clone(), self.agent_id.clone())
                })
                .collect::<Vec<ToolExecution>>();
        (user_message, reasoning_blocks, system_prompt, tool_executions)
    }
}
