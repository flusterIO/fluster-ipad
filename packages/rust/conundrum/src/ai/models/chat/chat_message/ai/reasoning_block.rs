use std::sync::Arc;

use fake::Dummy;
use rig::completion::message::{Reasoning, ReasoningContent};
use serde::{Deserialize, Serialize};

use crate::{
    ai::rig::ai_traits::from_with_convo_information::FromWithConvoInformation,
    ecosystem::db::{
        db_traits::{
            db_entity::{DBEntity, DBSchema},
            db_field::DatabaseField,
        },
        tables::DatabaseTable,
    },
    impl_default_crud,
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId, static_id::StaticId},
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct ReasoningBlock {
    pub id: DatabaseId,
    pub convo_id: DatabaseId,
    pub agent_id: DatabaseId,
    pub content: String,
    pub ctime: DateTime,
}

impl FromWithConvoInformation<rig::completion::message::Reasoning> for ReasoningBlock {
    fn from_with_convo_info(data: Reasoning, convo_id: DatabaseId, agent_id: Option<DatabaseId>) -> Self {
        let content = data.content
                          .iter()
                          .map(|x| match x {
                              ReasoningContent::Summary(x) => x.clone(),
                              ReasoningContent::Text { text,
                                                       .. } => text.clone(),
                              _ => String::new(),
                          })
                          .collect::<Vec<String>>()
                          .join("");

        Self { id: DatabaseId::new(),
               convo_id,
               agent_id: agent_id.unwrap_or_else(|| StaticId::DefaultAgent.into()),
               content,
               ctime: DateTime::new_now() }
    }
}

impl<'a> DBSchema<'a> for ReasoningBlock {
    fn arrow_fields(
        )
        -> crate::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(DatabaseId::field_definition("convo_id", false)),
                Arc::new(DatabaseId::field_definition("agent_id", false)),
                Arc::new(String::field_definition("content", false)),
                Arc::new(DateTime::field_definition("ctime", false))])
    }
}

impl<'a> DBEntity<'a, DatabaseId> for ReasoningBlock {
    type PartialUpdateType = ReasoningBlock;

    fn table() -> crate::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::AgentReasoning
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

impl_default_crud!(ReasoningBlock, ReasoningBlock, DatabaseId);
