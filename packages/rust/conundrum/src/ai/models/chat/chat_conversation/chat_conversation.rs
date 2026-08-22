use std::ops::{Index, IndexMut};
use std::sync::Arc;

use crate::ecosystem::db::db::ArcMutexDB;
use crate::ecosystem::db::db_traits::db_entity::{DBEntity, DBSchema};
use crate::ecosystem::db::db_traits::db_identifiable::DatabaseIdentifiable;
use crate::ecosystem::db::db_traits::entity_crud::EntityCRUD;
use crate::ecosystem::db::parameters::general::pagination::PaginationParams;
use crate::ecosystem::db::tables::DatabaseTable;
use crate::ecosystem::error_handling::db_error::DatabaseResult;
use crate::impl_default_crud;
use crate::lifted_models::primitives::date_time::DateTime;
use crate::lifted_models::primitives::db_id::DatabaseId;
use crate::{
    ai::models::chat::chat_conversation::chat_conversation_partial::ChatConversationPartial,
    ecosystem::db::db_traits::db_field::DatabaseField,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct ChatConversation {
    pub id: DatabaseId,
    pub label: String,
    pub desc: Option<String>,
    pub requires_label_update: bool,
    pub ctime: DateTime,
    #[serde(default = "DateTime::new_now")]
    pub utime: DateTime,
}

impl ChatConversation {
    pub fn new(convo_id: DatabaseId, label: Option<String>) -> Self {
        Self { id: convo_id,
               label: String::new(),
               desc: None,
               requires_label_update: true,
               ctime: DateTime::new_now(),
               utime: DateTime::new_now() }
    }

    /// Required over the merge method to preserve the conversation ctime.
    pub async fn make_require_update(&self, db: &ArcMutexDB) -> DatabaseResult<()> {
        let predicate = self.id.to_predicate("id");
        let mut _self = ChatConversation::get_by_predicate(Some(predicate),
                                                           Some(PaginationParams::single()),
                                                           None,
                                                           &Arc::clone(db)).await?;
        match _self.clone().len() {
            0 => {
                log::info!("Chat Conversation not found. Creating a new one.");
                ChatConversation::save_many(vec![self.clone()], &Arc::clone(db)).await?;
            }
            1 => {
                let item = _self.index_mut(0);
                if !item.requires_label_update {
                    item.requires_label_update = true;
                    Self::save_many(vec![item.clone()], &Arc::clone(db)).await?;
                }
            }
            _ => {
                log::error!("Found multiple chat conversation's with the same id. Shit's gone haywire.")
            }
        };
        Ok(())
    }
}

impl_default_crud!(ChatConversation, ChatConversationPartial, DatabaseId);

impl<'a> DBSchema<'a> for ChatConversation {
    fn arrow_fields(
        )
        -> crate::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("label", false)),
                Arc::new(String::field_definition("desc", true)),
                Arc::new(bool::field_definition("requires_label_update", true)),
                Arc::new(DateTime::field_definition("ctime", false)),
                Arc::new(DateTime::field_definition("utime", false))])
    }
}

impl<'a> DBEntity<'a, DatabaseId> for ChatConversation {
    type PartialUpdateType = ChatConversation;

    fn table() -> crate::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::ChatConversation
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

    fn set_primary_value(&mut self, value: DatabaseId) {
        self.id = value.clone();
    }
}
