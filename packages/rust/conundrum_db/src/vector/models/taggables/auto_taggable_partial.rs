use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    date_time::date_time::DateTime, primitives::db_id::DatabaseId, taggables::taggable::TaggableVariant,
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct AutoTaggablePartial {
    pub id: DatabaseId,
    pub value: Option<String>,
    pub variant: Option<TaggableVariant>,
    pub glob: Option<String>,
    pub utime: Option<DateTime>,
}

impl<'a> DBSchema<'a> for AutoTaggablePartial {}
