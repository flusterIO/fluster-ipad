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
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    ai::{ai_generated_status::AIGeneratedStatus, ai_interactions::AIInteractions},
    taggables::taggables::Taggables,
    text::text_based_content::text_based_content_trait::TextBasedContent as TextBasedContentTrait,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextBasedContent<ContentType, ChunkType, ParseParameters>
    where ContentType: TextBasedContentTrait<ParseParameters, ChunkType> + Serialize + Debug,
          ChunkType: Serialize + Debug {
    pub id: DatabaseId,
    pub content: ContentType,
    pub title: Option<String>,
    pub ai_generated: AIGeneratedStatus,
    pub taggables: Taggables,
    pub ws_path: Option<WorkspaceRelativeStringPath>,
    pub ctime: DateTime,
    pub utime: DateTime,
    pub ai: AIInteractions,
    pub x: PhantomData<ChunkType>,
    pub parse_params: PhantomData<ParseParameters>,
}

impl<ChunkType: Serialize + Debug,
     ParseParameters,
     ContentType: TextBasedContentTrait<ParseParameters, ChunkType> + Serialize + Debug> Dummy<Faker>
    for TextBasedContent<ContentType, ChunkType, ParseParameters>
{
    fn dummy_with_rng<R: fake::rand::prelude::RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        todo!()
    }
}

impl<'a,
     ChunkType: Serialize + Debug,
     ContentType: Serialize + Debug + TextBasedContentTrait<ParseParameters, ChunkType> + Dummy<Faker> + Deserialize<'a>,
     ParseParameters> DBSchema<'a> for TextBasedContent<ContentType, ChunkType, ParseParameters>
{
    fn arrow_fields() -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<Arc<Field>>> {
        let res = vec![Arc::new(DatabaseId::field_definition("id", false)),
                       Arc::new(String::field_definition_large("content", false)),
                       Arc::new(AIGeneratedStatus::field_definition("ai_generated", false)),
                       Arc::new(workspace_relative_path_field("ws_path", true)),
                       Arc::new(String::field_definition("title", true)),
                       Arc::new(DateTime::field_definition("ctime", false)),
                       Arc::new(DateTime::field_definition("utime", false)),
                       Arc::new(AIInteractions::field_definition("ai", false))];
        Ok(res)
    }
}
