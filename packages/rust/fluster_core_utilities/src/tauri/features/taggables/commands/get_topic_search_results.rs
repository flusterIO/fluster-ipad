use crate::{
    core_types::fluster_error::{FlusterError, FlusterResult},
    tauri::features::taggables::data::taggable_search_results::TraditionalSearchResults,
};

#[tauri::command]
#[specta::specta]
pub async fn get_topic_search_results(
    tag_values: Vec<String>,
) -> FlusterResult<TraditionalSearchResults> {
    Err(FlusterError::NotImplemented)
}
