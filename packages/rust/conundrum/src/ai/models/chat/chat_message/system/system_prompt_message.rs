use std::sync::Arc;

use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::{
    ai::rig::ai_traits::from_with_convo_information::FromWithConvoInformation,
    ecosystem::db::{
        db_traits::{
            db_entity::{DBEntity, DBSchema},
            db_field::DatabaseField,
        },
        macros::impl_default_crud,
        tables::DatabaseTable,
    },
    impl_default_crud,
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::SystemPromptMessage, source_crate = true)]
pub struct SystemPromptMessage {
    pub id: DatabaseId,
    pub body: String,
    pub convo_id: DatabaseId,
    pub ctime: DateTime,
}

impl FromWithConvoInformation<String> for SystemPromptMessage {
    fn from_with_convo_info(data: String, convo_id: DatabaseId, agent_id: Option<DatabaseId>) -> Self {
        Self { id: DatabaseId::new(),
               body: data,
               convo_id,
               ctime: DateTime::new_now() }
    }
}
