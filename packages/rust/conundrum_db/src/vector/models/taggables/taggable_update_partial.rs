use std::sync::Arc;

use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_traits::db_field::DatabaseField,
    models::{
        date_time::date_time::DateTime, primitives::case_insensitive_string::CaseInsensitiveString,
        taggables::tag_location::TagLocation,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct TaggablePartial {
    /// The value will never be updated, only used for comparison.
    pub value: String,
    pub location: Option<TagLocation>,
    pub last_access: Option<DateTime>,
}

impl<'a> DBSchema<'a> for TaggablePartial {}
