use std::sync::Arc;

use arrow_schema::Field;
use conundrum::{
    ai::rig::features::chat::convo_context::ArcMutexConversationContext,
    ecosystem::{
        db::{
            db::ArcMutexDB,
            db_traits::{
                db_entity::{DBEntity, DBSchema},
                db_identifiable::DatabaseIdentifiable,
                entity_crud::EntityCRUD,
            },
            parameters::general::pagination::PaginationParams,
            tables::DatabaseTable,
        },
        error_handling::db_error::DatabaseResult,
    },
    impl_default_crud,
    lang::runtime::run_conundrum::ParseConundrumOptions,
    lifted_models::primitives::db_id::DatabaseId,
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    meta::front_matter::front_matter::FrontMatter,
    text::{
        cdrm::cdrm_content::CdrmContent,
        text_based_content::{text_based_chunk::TextBasedChunk, text_based_content::TextBasedContent},
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy)]
#[serde(transparent)]
pub struct CdrmModel(pub TextBasedContent<CdrmContent, TextBasedChunk, ParseConundrumOptions>);

impl<'a> DBSchema<'a> for CdrmModel {
    fn arrow_fields() -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<Arc<Field>>> {
        TextBasedContent::<CdrmContent, TextBasedChunk, ParseConundrumOptions>::arrow_fields()
    }
}

impl<'a> DBEntity<'a, DatabaseId> for CdrmModel {
    type PartialUpdateType = CdrmModel;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::Cdrm
    }

    fn merge_keys() -> &'static [&'static str] {
        &["id"]
    }

    fn primary_key() -> &'static str {
        "id"
    }

    fn primary_value(&self) -> DatabaseId {
        self.0.id.clone()
    }

    fn set_primary_value(&mut self, value: DatabaseId) {
        self.0.id = value.clone();
    }
}

impl_default_crud!(CdrmModel, CdrmModel, DatabaseId);

impl CdrmModel {
    pub async fn get_related_frontmatter(&self, db: ArcMutexDB) -> DatabaseResult<Option<FrontMatter>> {
        let predicate = self.0.id.to_predicate("note_id");
        let item = FrontMatter::get_one_by_predicate(Some(predicate), None, &Arc::clone(&db)).await?;
        Ok(item)
    }
}
