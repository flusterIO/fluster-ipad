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
    text::text_based_content::text_based_content_trait::TextBasedContent as TextBasedContentTrait,
};

#[derive(Serialize, Deserialize, Clone, Debug, DBSchema, DBPartial, specta::Type)]
#[db(include_partial_generics)]
pub struct TextBasedContent<'a, ContentType, ChunkType, ParseParameters>
    where ContentType: Serialize + Debug + Deserialize<'a> + Dummy<Faker>,
          ChunkType: Serialize + Debug + Deserialize<'a>,
          Self: Deserialize<'a> {
    pub id: DatabaseId,
    pub content: ContentType,
    pub title: Option<String>,
    pub ai_generated: AIGeneratedStatus,
    pub ws_root: Option<String>,
    pub relative_path: Option<String>,
    pub ctime: DateTime,
    pub utime: DateTime,
    pub last_sync: Option<DateTime>,
    pub ai: AIInteractions,
    #[db(partial(skip_fields))]
    #[serde(default)]
    pub chunk_type: PhantomData<ChunkType>,
    #[db(partial(skip_fields))]
    #[serde(default)]
    pub parse_params: PhantomData<ParseParameters>,
}

impl<'a,
     ContentType: TextBasedContentTrait<ParseParameters, ChunkType> + Serialize + Debug,
     ChunkType: Serialize + Debug + Deserialize<'a>,
     ParseParameters> Dummy<Faker> for TextBasedContent<'a, ContentType, ChunkType, ParseParameters>
{
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        todo!()
    }
}

impl<'a, ContentType, ChunkType, ParseParams> TextBasedContent<'a, ContentType, ChunkType, ParseParams>
    where ChunkType: Serialize + Debug + Deserialize<'a>,
          ContentType:
              TextBasedContentTrait<ParseParams, ChunkType> + Serialize + Debug + Deserialize<'a> + Dummy<Faker>
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
                           ai: AIInteractions::default(),
                           chunk_type: PhantomData::<ChunkType>::default(),
                           parse_params: PhantomData::<ParseParams>::default() }
    }
}
