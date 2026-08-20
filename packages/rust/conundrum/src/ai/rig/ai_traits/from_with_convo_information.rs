use crate::{ecosystem::error_handling::db_error::DatabaseResult, lifted_models::primitives::db_id::DatabaseId};

pub trait FromWithConvoInformation<T, AgentIdType = Option<DatabaseId>> {
    fn from_with_convo_info(data: T, convo_id: DatabaseId, agent_id: AgentIdType) -> Self;
}

pub trait TryFromWithConvoInformation<T, AgentIdType = Option<DatabaseId>> {
    fn try_from_with_convo_info(data: T, convo_id: DatabaseId, agent_id: AgentIdType) -> DatabaseResult<Self>
        where Self: Sized;
}
