use conundrum::{
    ecosystem::db::db_traits::db_entity::DBSchema,
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::taggables::taggable::TaggableVariant;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct AutoTaggablePartial {
    pub id: DatabaseId,
    pub value: Option<String>,
    pub variant: Option<TaggableVariant>,
    pub glob: Option<String>,
    pub utime: Option<DateTime>,
}

impl<'a> DBSchema<'a> for AutoTaggablePartial {}
