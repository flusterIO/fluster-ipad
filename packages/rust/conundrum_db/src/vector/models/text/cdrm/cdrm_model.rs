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
                entity_crud::{EntityCRUD, filter_one},
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
use conundrum_macros::{DatabaseEntity, DatabaseModel};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    meta::front_matter::front_matter::FrontMatter,
    text::{
        cdrm::cdrm_content::CdrmContent,
        text_based_content::{text_based_chunk::TextBasedChunk, text_based_content::TextBasedContent},
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy, DatabaseEntity)]
#[serde(transparent)]
#[db(table = DatabaseTable::Cdrm, unit = TextBasedContent<CdrmContent, TextBasedChunk, ParseConundrumOptions>)]
pub struct CdrmModel(pub TextBasedContent<CdrmContent, TextBasedChunk, ParseConundrumOptions>);

impl CdrmModel {
    pub async fn get_related_frontmatter(&self, db: ArcMutexDB) -> DatabaseResult<Option<FrontMatter>> {
        let predicate = self.0.id.to_predicate("note_id");
        let items = FrontMatter::get_by_predicate(Some(predicate),
                                                  Some(PaginationParams::single()),
                                                  None,
                                                  Arc::clone(&db)).await?;
        let item = filter_one(items)?;
        Ok(item)
    }
}
