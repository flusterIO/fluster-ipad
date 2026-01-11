use crate::core_types::fluster_error::{FlusterError, FlusterResult};

#[tauri::command]
#[specta::specta]
pub async fn read_file_to_bytes(fs_path: String) -> FlusterResult<Vec<u8>> {
    if let Ok(exists) = std::fs::exists(&fs_path) {
        if exists {
            let res = tokio::fs::read(&fs_path)
                .await
                .map_err(|_| FlusterError::FailToReadFileSystemPath(fs_path.clone()))?;
            Ok(res)
        } else {
            Err(FlusterError::FailToReadFileSystemPath(fs_path.clone()))
        }
    } else {
        Err(FlusterError::FailToReadFileSystemPath(fs_path.clone()))
    }
}

#[tauri::command]
#[specta::specta]
pub async fn read_utf8_file(fs_path: String) -> FlusterResult<String> {
    if let Ok(exists) = std::fs::exists(&fs_path) {
        if exists {
            let res = tokio::fs::read_to_string(&fs_path)
                .await
                .map_err(|_| FlusterError::FailToReadFileSystemPath(fs_path.clone()))?;
            Ok(res)
        } else {
            Err(FlusterError::FailToReadFileSystemPath(fs_path.clone()))
        }
    } else {
        Err(FlusterError::FailToReadFileSystemPath(fs_path.clone()))
    }
}
