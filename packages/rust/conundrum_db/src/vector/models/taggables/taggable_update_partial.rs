use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_traits::{db_entity::ArrowSchemaRepresentable, db_field::DatabaseField},
    models::{
        date_time::date_time::DateTime, primitives::case_insensitive_string::CaseInsensitiveString,
        taggables::tag_location::TagLocation,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
pub struct TaggablePartial {
    /// The value will never be updated, only used for comparison.
    pub value: String,
    pub location: Option<TagLocation>,
    pub last_access: Option<DateTime>,
}

impl ArrowSchemaRepresentable for TaggablePartial {
    fn arrow_schema() -> std::sync::Arc<arrow_schema::Schema> {
        let val = CaseInsensitiveString::field_definition("value", false);
        Arc::new(lancedb::arrow::arrow_schema::Schema::new(vec![val,
                                                                TagLocation::field_definition("location", true),
                                                                DateTime::field_definition("last_access", true),]))
    }
}
