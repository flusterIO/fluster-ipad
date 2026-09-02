use rspc::Procedure;
use std::sync::Arc;

use conundrum::{
    ai::models::{
        chat::{
            chat_conversation::chat_conversation::ChatConversation,
            chat_message::{
                ai::{ai_message::AIMessage, reasoning_block::ReasoningBlock},
                user::user_message::UserMessage,
            },
        },
        tool::tool_execution::ToolExecution,
    },
    ecosystem::{
        db::db_traits::{db_entity::DBSchema, entity_crud::EntityCRUD},
        error_handling::server_error::ServerError,
    },
    lang::lib::std_lib_impls::json_string::QuotedString,
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState;

pub fn get_chat_conversation_crud() -> rspc::Router<Arc<ServerState>> {
    rspc::Router::<std::sync::Arc<conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState>>::new().procedure("get_by_predicate",Procedure::<std::sync::Arc<conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState>,conundrum_db::vector::parameters::general::general_query::GeneralQuery,Vec<ChatConversation>>::builder::<conundrum::ecosystem::error_handling::server_error::ServerError>().query(|state: std::sync::Arc<ServerState>,params: conundrum_db::vector::parameters::general::general_query::GeneralQuery|async move {
            let r =  <ChatConversation as EntityCRUD<<ChatConversation as DBSchema>::PartialUpdateType>>::get_by_predicate(params.predicate,Some(params.pagination),params.sort, state.db.clone()).await.map_err(|e|{
                log::error!("Error: {:?}",e);
                ServerError::DatabaseError(e)
            })?;
            Ok(r)
        })).procedure("save_many",Procedure::<std::sync::Arc<conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState>,Vec<ChatConversation>,()>::builder::<conundrum::ecosystem::error_handling::server_error::ServerError>().mutation(|state: std::sync::Arc<ServerState>,params: Vec<ChatConversation> |async move {
            <ChatConversation as EntityCRUD<<ChatConversation as DBSchema>::PartialUpdateType>>::save_many(params, state.db.clone()).await.map_err(|e|{
                log::error!("Error: {:?}",e);
                ServerError::DatabaseError(e)
            })?;
            Ok(())
        })).procedure("update_many",Procedure::<std::sync::Arc<conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState>,Vec<<ChatConversation as DBSchema>::PartialUpdateType>,()>::builder::<conundrum::ecosystem::error_handling::server_error::ServerError>().mutation(|state: std::sync::Arc<ServerState>,params: Vec<<ChatConversation as DBSchema>::PartialUpdateType> |async move {
            <ChatConversation as EntityCRUD<<ChatConversation as DBSchema>::PartialUpdateType>>::merge_by_primary_key(params, state.db.clone()).await.map_err(|e|{
                log::error!("Error: {:?}",e);
                ServerError::DatabaseError(e)
            })?;
            Ok(())
        })).procedure("delete_by_predicate",Procedure::<std::sync::Arc<conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState>,String,()>::builder::<conundrum::ecosystem::error_handling::server_error::ServerError>().mutation(|state: std::sync::Arc<ServerState>,params: String|async move {
            <ChatConversation as EntityCRUD<<ChatConversation as DBSchema>::PartialUpdateType>>::delete_by_predicate(params.as_str(), state.db.clone()).await.map_err(|e|{
                log::error!("Error: {:?}",e);
                ServerError::DatabaseError(e)
            })?;
            Ok(())
        })).procedure("delete_conversation", Procedure::<Arc<ServerState>, DatabaseId, ()>::builder::<conundrum::ecosystem::error_handling::server_error::ServerError>().mutation(|state: std::sync::Arc<ServerState>, params: DatabaseId | async move {
        let predicate = format!("convo_id = {}", params.to_quoted_string()?);
            <ChatConversation as EntityCRUD<<ChatConversation as DBSchema>::PartialUpdateType>>::delete_by_predicate(format!("id = {}", &params.to_quoted_string()?).as_str(), state.db.clone()).await?;
            <UserMessage as EntityCRUD<<UserMessage as DBSchema>::PartialUpdateType>>::delete_by_predicate(predicate.as_str(), state.db.clone()).await?;
            <AIMessage as EntityCRUD<<AIMessage as DBSchema>::PartialUpdateType>>::delete_by_predicate(predicate.as_str(), state.db.clone()).await?;
            <ReasoningBlock as EntityCRUD<<ReasoningBlock as DBSchema>::PartialUpdateType>>::delete_by_predicate(predicate.as_str(), state.db.clone()).await?;
            <ToolExecution as EntityCRUD<<ToolExecution as DBSchema>::PartialUpdateType>>::delete_by_predicate(predicate.as_str(), state.db.clone()).await?;
            Ok(())
        }))
}
