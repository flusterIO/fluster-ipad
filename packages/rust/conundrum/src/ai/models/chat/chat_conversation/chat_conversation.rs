use std::ops::IndexMut;
use std::sync::Arc;

use crate::ai::models::chat::chat_conversation::vector_mode::VectorMode;
use crate::ecosystem::db::db::ArcMutexDB;
use crate::ecosystem::db::db_traits::db_entity::{DBEntity, DBSchema};
use crate::ecosystem::db::db_traits::db_field::DatabaseField;
use crate::ecosystem::db::db_traits::db_identifiable::DatabaseIdentifiable;
use crate::ecosystem::db::db_traits::entity_crud::EntityCRUD;
use crate::ecosystem::db::db_traits::into_partial::IntoPartial;
use crate::ecosystem::db::parameters::general::pagination::PaginationParams;
use crate::ecosystem::db::tables::DatabaseTable;
use crate::ecosystem::error_handling::db_error::DatabaseResult;
use crate::impl_default_crud;
use crate::lifted_models::primitives::date_time::DateTime;
use crate::lifted_models::primitives::db_id::DatabaseId;
use conundrum_macros::DatabaseEntity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, fake::Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::ChatConversation, source_crate = true)]
pub struct ChatConversation {
    pub id: DatabaseId,
    pub label: String,
    pub desc: Option<String>,
    pub requires_label_update: bool,
    pub ctime: DateTime,
    #[serde(default = "DateTime::new_now")]
    pub utime: DateTime,
}

impl From<ChatConversation> for <ChatConversation as DBEntity>::PartialUpdateType {
    fn from(value: ChatConversation) -> Self {
        Self { id: value.id.clone(),
               label: Some(value.label.clone()),
               desc: Some(value.desc.clone()),
               requires_label_update: Some(value.requires_label_update),
               ctime: Some(value.ctime),
               utime: Some(value.utime) }
    }
}

impl ChatConversation {
    pub fn new(convo_id: DatabaseId, label: Option<String>) -> Self {
        Self { id: convo_id,
               label: label.unwrap_or(String::from("New Chat")),
               desc: None,
               requires_label_update: true,
               ctime: DateTime::new_now(),
               utime: DateTime::new_now() }
    }

    /// Required over the merge method to preserve the conversation ctime.
    pub async fn make_require_update(&self, db: ArcMutexDB) -> DatabaseResult<()> {
        let predicate = self.id.to_predicate("id");
        let mut _self = ChatConversation::get_by_predicate(Some(predicate),
                                                           Some(PaginationParams::single()),
                                                           None,
                                                           Arc::clone(&db)).await?;
        match _self.clone().len() {
            0 => {
                log::info!("Chat Conversation not found. Creating a new one.");
                ChatConversation::save_many(vec![self.clone()], Arc::clone(&db)).await?;
            }
            1 => {
                let item = _self.index_mut(0);
                item.requires_label_update = true;
                item.utime = DateTime::new_now();
                let partial = <ChatConversation as DBEntity>::PartialUpdateType::from(item.clone());
                ChatConversation::merge_by_primary_key(vec![partial], Arc::clone(&db)).await?;
                log::info!("Updated one chat conversation.")
            }
            _ => {
                log::error!("Found multiple chat conversation's with the same id. Shit's gone haywire.")
            }
        };
        Ok(())
    }
}
