#[tauri::command]
#[specta::specta]
pub async fn path_exists(file_path: String) -> bool {
    std::fs::exists(file_path).is_ok_and(|x| x)
}
