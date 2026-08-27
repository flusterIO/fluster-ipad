use conundrum::{lifted_models::primitives::db_id::DatabaseId, output::parsing_result::tag_result::TagResult};

use crate::vector::models::taggables::{auto_taggable::AutoTaggable, tag_location::TagLocation};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum TagSource {
    Cdrm,
    Notebook,
    Typst,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct GenericTagInput {
    pub value: String,
    pub source_id: DatabaseId,
    pub source_type: TagSource,
    pub location: TagLocation,
}

impl GenericTagInput {
    pub fn from_auto_taggable(taggable: AutoTaggable, source_id: DatabaseId, source_type: TagSource) -> Self {
        GenericTagInput { value: taggable.value.clone(),
                          source_id: source_id.clone(),
                          source_type,
                          location: TagLocation::AppInserted }
    }

    pub fn from_tag_result(value: TagResult, source_id: DatabaseId, location: TagLocation) -> Self {
        GenericTagInput { value: value.body.clone(),
                          source_id,
                          source_type: TagSource::Cdrm,
                          location }
    }
}
