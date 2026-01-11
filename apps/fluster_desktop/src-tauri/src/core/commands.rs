use std::path::PathBuf;

use fluster_core_utilities::{
    core_types::fluster_error::{FlusterError, FlusterResult},
    tauri::features::{
        database, file_system,
        syncing::data::sync_file_system_options::SyncFilesystemDirectoryOptions,
    },
};

// -- File System --

#[tauri::command]
#[specta::specta]
pub async fn path_exists(file_path: String) -> bool {
    file_system::path_exists::path_exists(file_path).await
}

#[tauri::command]
#[specta::specta]
pub async fn read_utf8_file(fs_path: String) -> FlusterResult<String> {
    file_system::read_file::read_utf8_file(fs_path).await
}

#[tauri::command]
#[specta::specta]
pub async fn read_file_to_bytes(fs_path: String) -> FlusterResult<Vec<u8>> {
    file_system::read_file::read_file_to_bytes(fs_path).await
}

// -- Database --

#[tauri::command]
#[specta::specta]
pub fn get_database_path() -> FlusterResult<PathBuf> {
    database::database_utils::get_database_path()
}
