use std::path::PathBuf;

use fluster_core_utilities::{
    core_types::fluster_error::FlusterResult,
    tauri::features::{
        database, file_system,
        health::{self, data::desktop_health_report::DesktopHealthReport},
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

#[tauri::command]
#[specta::specta]
pub async fn fs_file_extension_glob(
    file_extension: String,
    base_path: String,
    n_threads: String,
) -> FlusterResult<Vec<String>> {
    file_system::glob_utils::fs_file_extension_glob(file_extension, base_path, n_threads).await
}

// -- Database --

#[tauri::command]
#[specta::specta]
pub fn get_database_path() -> FlusterResult<PathBuf> {
    database::database_utils::get_database_path()
}

#[tauri::command]
#[specta::specta]
pub async fn initialize_database() -> FlusterResult<()> {
    database::initialize_database::initialize_database().await
}

// -- Health ---

#[tauri::command]
#[specta::specta]
pub async fn get_desktop_health_report() -> DesktopHealthReport {
    health::methods::get_desktop_health_report::get_desktop_health_report().await
}
