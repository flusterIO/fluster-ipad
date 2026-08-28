use std::sync::Arc;

use conundrum::ai::models::chat::chat_message::ai::ai_message::AIMessage;
use conundrum::ai::models::chat::chat_message::system::system_prompt_message::SystemPromptMessage;
use conundrum::ai::models::chat::chat_message::user::user_message::UserMessage;
use conundrum::ai::models::chat::client_chat_data::client_chat_data::ClientChatData;
use conundrum::ai::models::tool::tool_execution::ToolExecution;
use conundrum::ecosystem::db::db_traits::db_identifiable::DatabaseIdentifiable;
use conundrum::ecosystem::db::db_traits::entity_crud::EntityCRUD;
use conundrum::ecosystem::db::parameters::general::pagination::PaginationParams;
use conundrum::ecosystem::db::parameters::general::sort_query::SortQuery;
use conundrum::ecosystem::error_handling::server_error::ServerError;
use conundrum::{
    ai::models::chat::chat_message::ai::reasoning_block::ReasoningBlock,
    ecosystem::db::parameters::ai::chat_history_parameters::ChatHistoryParams,
};
use conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState;
use rspc::{Procedure, Router};

pub fn get_agent_router() -> Router<Arc<ServerState>> {
    Router::<Arc<ServerState>>::new().procedure("save_chat_data",
                                            Procedure::<Arc<ServerState>, ClientChatData, ()>::builder::<ServerError>().mutation(|state: Arc<ServerState>, req: ClientChatData| async move {
                                                let (user_message, reasoning_blocks, system_prompt, tool_executions) = req.expand();
                                                UserMessage::save_many(vec![user_message], &Arc::clone(&state.db)).await?;
                                                ReasoningBlock::save_many(reasoning_blocks
                                                    , &Arc::clone(&state.db)).await?;
                                                if let Some(sp) = system_prompt {
                                                    SystemPromptMessage::save_many(vec![sp], &Arc::clone(&state.db)).await?;
                                                }
                                                ToolExecution::save_many(tool_executions, &Arc::clone(&state.db)).await?;
                                                Ok(())
                                                                               }))
.procedure("load_chat_history",
                                            Procedure::<Arc<ServerState>, ChatHistoryParams, (Vec<UserMessage>, Vec<SystemPromptMessage>, Vec<AIMessage>, Vec<ReasoningBlock>, Vec<ToolExecution>)>::builder::<ServerError>().query(|state: Arc<ServerState>, req: ChatHistoryParams| async move {
                                                let predicate = req.convo_id.to_predicate("convo_id");
                                                let pag = Some(PaginationParams {
                                                    page: 1,
                                                    per_page: req.max_count
                                                });
                                                let sort = Some(vec![SortQuery::order_by_ctime()]);
                                                let user_messages = UserMessage::get_by_predicate(Some(predicate.clone()), pag.clone(), sort.clone(), Arc::clone(&state.db)).await?;
                                                let system_prompts = SystemPromptMessage::get_by_predicate(Some(predicate.clone()), pag.clone(), sort.clone(), Arc::clone(&state.db)).await?;
                                                let agent_reasoning = ReasoningBlock::get_by_predicate(Some(predicate.clone()), pag.clone(), sort.clone(), Arc::clone(&state.db)).await?;
                                                let agent_messages = AIMessage::get_by_predicate(Some(predicate.clone()), pag.clone(), sort.clone(), Arc::clone(&state.db)).await?;
                                                let tool_executions = ToolExecution::get_by_predicate(Some(predicate.clone()), pag.clone(), sort.clone(), Arc::clone(&state.db)).await?;
                                                Ok((user_messages, system_prompts, agent_messages, agent_reasoning, tool_executions))
                                                                               }))
}
