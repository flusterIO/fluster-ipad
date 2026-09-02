use std::sync::Arc;

use arrow_schema::Field;
use conundrum::{
    ai::rig::features::chat::convo_context::ArcMutexConversationContext,
    ecosystem::{
        db::{
            db_client::db_client::DBClient,
            db_traits::{
                db_entity::{DBEntity, DBSchema},
                db_identifiable::DatabaseIdentifiable,
                entity_crud::{EntityCRUD, filter_one},
                into_partial::IntoPartial,
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
        text_based_content::{
            text_based_chunk::TextBasedChunk, text_based_content::TextBasedContent,
            text_based_content_trait::TextBasedContent as TextBasedContentTrait,
        },
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy, DatabaseEntity)]
#[serde(transparent)]
#[db(table = DatabaseTable::Cdrm, unit = TextBasedContent<'a, CdrmContent, TextBasedChunk, ParseConundrumOptions>, use_type_path = true)]
pub struct CdrmModel(pub TextBasedContent<CdrmContent, TextBasedChunk, ParseConundrumOptions>);

impl IntoPartial<<CdrmModel as DBSchema>::PartialUpdateType> for CdrmModel {
    fn into_partial(&self) -> <CdrmModel as DBSchema>::PartialUpdateType {
        CdrmModelPartial(TextBasedContent::<CdrmContent, TextBasedChunk, ParseConundrumOptions>::new_partial(Some(self.0
                                                                                                     .content
                                                                                                     .clone()),
                                                                                            self.0.title.clone(),
                                                                                            self.0.ws_root.clone(),
                                                                                            self.0.relative_path.clone()))
    }
}

impl CdrmModel {
    pub async fn get_related_frontmatter(&self, db: DBClient) -> DatabaseResult<Option<FrontMatter>> {
        let predicate = self.0.id.to_predicate("note_id");
        let items = <FrontMatter as EntityCRUD<<FrontMatter as DBSchema>::PartialUpdateType>>::get_by_predicate(Some(predicate),
                                                                  Some(PaginationParams::single()),
                                                                  None,
                                                                  db.clone()).await?;
        let item = filter_one(items)?;
        Ok(item)
    }
}
