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
use conundrum_macros::{DBPartial, DBSchema};
use fake::{Dummy, Faker};
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    ai::{ai_generated_status::AIGeneratedStatus, ai_interactions::AIInteractions},
    binary::{
        binary::Binary, binary_based_content_trait::BinaryBasedContent as BinaryBasedContentTrait,
        binary_based_content_wrapper_trait::BinaryBasedContentWrapper,
    },
    taggables::taggables::Taggables,
    text::text_based_content::text_based_content_trait::TextBasedContent as TextBasedContentTrait,
};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, specta::Type, DBSchema, DBPartial)]
pub struct BinaryBasedContent<ContentType, ChunkType, ParseParameters>
    where ContentType: BinaryBasedContentTrait<ParseParameters, ChunkType> + Serialize + Debug,
          Self: BinaryBasedContentWrapper,
          ChunkType: Serialize + Debug {
    pub id: DatabaseId,
    pub content: <Self as BinaryBasedContentWrapper>::ContentType,
    pub title: Option<String>,
    pub ai_generated: AIGeneratedStatus,
    pub taggables: Taggables,
    pub ws_path: Option<WorkspaceRelativeStringPath>,
    pub ctime: DateTime,
    pub utime: DateTime,
    pub ai: AIInteractions,
}

impl<ContentType: BinaryBasedContentTrait<ParseParameters, ChunkType> + Serialize + Debug,
     ChunkType: Serialize + Debug,
     ParseParameters> Dummy<Faker> for BinaryBasedContent<ContentType, ChunkType, ParseParameters>
{
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        todo!()
    }
}

impl<ContentType, ChunkType, ParseParameters> BinaryBasedContentWrapper
    for BinaryBasedContent<ContentType, ChunkType, ParseParameters>
{
    type ContentType = ContentType;

    fn to_bytes(&self) -> Vec<u8> {
        self.content.bytes()
    }
}
