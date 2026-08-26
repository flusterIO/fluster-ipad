use crate::{
    ai::rig::features::chat::convo_context::{ArcMutexConversationContext, ConversationContext},
    ecosystem::error_handling::db_error::DatabaseResult,
    lifted_models::primitives::db_id::DatabaseId,
};

pub trait FromWithConvoInformation<T, AgentIdType = Option<DatabaseId>> {
    fn from_with_convo_info(data: T, convo_id: DatabaseId, agent_id: AgentIdType) -> Self;
}

pub trait TryFromWithConvoContext<T, AgentIdType = Option<DatabaseId>> {
    async fn try_from_with_convo_info(data: T, ctx: &ArcMutexConversationContext) -> DatabaseResult<Self>
        where Self: Sized;
}
