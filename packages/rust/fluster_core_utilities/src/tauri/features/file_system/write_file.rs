use crate::core_types::fluster_error::{FlusterError, FlusterResult};

#[tauri::command]
#[specta::specta]
pub async fn save_utf8_file(fs_path: String, file_content: String) -> FlusterResult<()> {
    tokio::fs::write(&fs_path, file_content)
        .await
        .map_err(|_| FlusterError::FailToSaveFile(fs_path.clone()))
}
