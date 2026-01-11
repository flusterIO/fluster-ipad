use serde::{Deserialize, Serialize};
use specta::Type;

use crate::tauri::features::taggables::data::models::shared_taggable_model::SharedTaggableModel;

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
pub struct AllTaggableData {
    pub tags: Vec<SharedTaggableModel>,
    pub topics: Vec<SharedTaggableModel>,
    pub subjects: Vec<SharedTaggableModel>,
}
