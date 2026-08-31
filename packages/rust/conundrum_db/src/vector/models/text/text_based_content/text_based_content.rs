use std::{fmt::Debug, marker::PhantomData, sync::Arc};

use arrow_schema::Field;
use conundrum::{
    ecosystem::db::db_traits::{
        db_entity::DBSchema,
        db_field::{DatabaseField, DatabaseFieldLarge},
        impls::workspace_relative_path_field::workspace_relative_path_field,
    },
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use conundrum_fs::models::user_workspace::workspace_relative_path_strings::WorkspaceRelativeStringPath;
use conundrum_macros::{DBPartial, DBSchema, DatabaseEntity, DatabaseModel};
use fake::{Dummy, Faker};
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    ai::{ai_generated_status::AIGeneratedStatus, ai_interactions::AIInteractions},
    taggables::taggables::Taggables,
    text::text_based_content::{
        text_based_content_trait::TextBasedContent as TextBasedContentTrait,
        text_based_content_wrapper::TextBasedContentWrapper,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, DBSchema, DBPartial, specta::Type, Dummy)]
pub struct TextBasedContent<'a, ChunkType, ParseParameters>
    where ChunkType: Serialize + Debug + Deserialize<'a> + Dummy<Faker> + Debug,
          Self: TextBasedContentWrapper<'a, ParseParameters, ChunkType> {
    pub id: DatabaseId,
    pub content: <Self as TextBasedContentWrapper<'a, ParseParameters, ChunkType>>::ContentType,
    pub title: Option<String>,
    pub ai_generated: AIGeneratedStatus,
    pub ws_root: Option<String>,
    pub relative_path: Option<String>,
    pub ctime: DateTime,
    pub utime: DateTime,
    pub last_sync: Option<DateTime>,
    pub ai: AIInteractions,
    // #[db(skip)]
    // #[serde(default)]
    // pub chunk_type: PhantomData<ChunkType>,
    // #[db(skip)]
    // #[serde(default)]
    // pub parse_params: PhantomData<ParseParameters>,
}

impl<'a,
     ContentType: TextBasedContentTrait<ParseParameters, ChunkType> + Serialize + Debug,
     ChunkType: Serialize + Debug,
     ParseParameters> Dummy<Faker> for TextBasedContent<'a, ChunkType, ParseParameters>
{
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        todo!()
    }
}

impl<'a, ContentType, ChunkType, ParseParams> TextBasedContent<'a, ChunkType, ParseParams>
    where ChunkType: Serialize + Debug,
          ContentType: TextBasedContentTrait<ParseParams, ChunkType> + Serialize + Debug
{
    pub fn new(content: ContentType, title: Option<String>, workspace_path: String, relative_path: String) -> Self {
        TextBasedContent { id: DatabaseId::new(),
                           content,
                           title,
                           ai_generated: AIGeneratedStatus::None,
                           ws_root: Some(workspace_path),
                           relative_path: Some(relative_path),
                           ctime: DateTime::new_now(),
                           utime: DateTime::new_now(),
                           last_sync: None,
                           ai: AIInteractions::default() /* chunk_type: PhantomData::default(),
                                                          * parse_params: PhantomData::default() */ }
    }
}

impl<'a, ContentType, ChunkType, ParseParameters> TextBasedContentWrapper
    for TextBasedContent<'a, ChunkType, ParseParameters>
{
    type ContentType = ContentType;

    fn inner_text(&self) -> String {
        let s = self.content.inner_text();
        s
    }
}
