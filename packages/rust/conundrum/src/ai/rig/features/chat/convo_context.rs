use std::sync::Arc;

use dashmap::DashMap;

use crate::{
    ecosystem::error_handling::db_error::DatabaseResult,
    lifted_models::primitives::{db_id::DatabaseId, static_id::StaticId},
};

#[derive(Clone, Debug)]
pub struct ConversationContext {
    pub convo: DatabaseId,
    pub agent: DatabaseId,
    pub accumulator: Arc<tokio::sync::Mutex<DashMap<DatabaseId, String>>>,
    pub reasoning_accumulator: Arc<tokio::sync::Mutex<DashMap<DatabaseId, String>>>,
}

impl ConversationContext {
    pub async fn clear_current_reasoning_accumulator(&mut self) {
        let accumulator = self.reasoning_accumulator.clone().lock_owned().await;
        accumulator.remove(&self.convo);
    }

    pub async fn clear_current_accumulator(&mut self) {
        let accumulator = self.accumulator.clone().lock_owned().await;
        accumulator.remove(&self.convo);
    }

    pub fn agent_id_if_not_default(&self) -> Option<DatabaseId> {
        let default_id: DatabaseId = StaticId::DefaultAgent.into();
        if default_id == self.agent {
            None
        } else {
            Some(self.agent.clone())
        }
    }

    pub async fn read_current_conversation(&self) -> DatabaseResult<String> {
        let accumulator = self.accumulator.clone().lock_owned().await;
        if let Some(res) = accumulator.get(&self.convo) {
            Ok(res.to_string())
        } else {
            log::error!("Failed to read accumulator for the current conversation.");
            Err(crate::ecosystem::error_handling::db_error::DatabaseError::SerializationError)
        }
    }
}

pub type ArcMutexConversationContext = Arc<tokio::sync::Mutex<ConversationContext>>;
