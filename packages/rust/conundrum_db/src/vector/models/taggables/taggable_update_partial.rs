use std::sync::Arc;

use conundrum::{ecosystem::db::db_traits::db_entity::DBSchema, lifted_models::primitives::date_time::DateTime};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::taggables::tag_location::TagLocation;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct TaggablePartial {
    /// The value will never be updated, only used for comparison.
    pub value: String,
    pub location: Option<TagLocation>,
    pub last_access: Option<DateTime>,
}

impl<'a> DBSchema<'a> for TaggablePartial {}
