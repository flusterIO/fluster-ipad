use crossbeam_channel::unbounded;
use fluster_core_utilities::core_types::fluster_error::{FlusterError, FlusterResult};
use glob::MatchOptions;
use ignore::{DirEntry, WalkBuilder, WalkState};

#[tauri::command]
#[specta::specta]
pub async fn fs_file_extension_glob(
    file_extension: String,
    base_path: String,
    n_threads: String,
) -> FlusterResult<Vec<String>> {
    let (sender, receiver) = unbounded::<String>();
    let threads: usize = n_threads.parse().unwrap();
    WalkBuilder::new(base_path)
        .threads(threads)
        .add_custom_ignore_filename(".flusterIgnore")
        .git_ignore(false)
        .ignore(true)
        .build_parallel()
        .run(|| {
            let _sender = sender.clone();
            let file_extension = file_extension.clone();
            Box::new(move |either_entry: Result<DirEntry, ignore::Error>| {
                if either_entry.is_ok() {
                    let entry = either_entry.unwrap();
                    let path = entry.path();
                    if path.is_file() && path.extension() == Some(file_extension.as_ref()) {
                        if let Some(path_as_string) = path.to_str() {
                            _sender.send(path_as_string.to_string()).unwrap();
                        }
                    }
                }
                WalkState::Continue
            })
        });

    drop(sender);

    let mut items: Vec<String> = Vec::new();
    for x in receiver.iter() {
        items.push(x)
    }
    Ok(items)
}

#[tauri::command]
#[specta::specta]
pub async fn get_files_by_file_extensions(
    file_extensions: Vec<String>,
    base_path: String,
    n_threads: String,
) -> FlusterResult<Vec<String>> {
    let (sender, receiver) = unbounded::<String>();
    let threads: usize = n_threads.parse().unwrap();
    WalkBuilder::new(base_path)
        .threads(threads)
        .add_custom_ignore_filename(".flusterIgnore")
        .git_ignore(false)
        .ignore(true)
        .build_parallel()
        .run(|| {
            let _sender = sender.clone();
            let file_extensions = file_extensions.clone();
            Box::new(move |either_entry: Result<DirEntry, ignore::Error>| {
                if either_entry.is_ok() {
                    let entry = either_entry.unwrap();
                    let path = entry.path();
                    let path_file_extension = path.extension();
                    if path.is_file()
                        && path_file_extension.is_some()
                        && file_extensions
                            .iter()
                            // RESUME: Come back here and handle this type equality issue/*
                            .any(|x| {
                                *x == path_file_extension.unwrap().to_str().unwrap().to_string()
                            })
                    {
                        if let Some(path_as_string) = path.to_str() {
                            _sender.send(path_as_string.to_string()).unwrap();
                        }
                    }
                }
                WalkState::Continue
            })
        });

    drop(sender);

    let mut items: Vec<String> = Vec::new();
    for x in receiver.iter() {
        items.push(x)
    }
    Ok(items)
}

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

#[tauri::command]
#[specta::specta]
pub async fn save_utf8_file(fs_path: String, file_content: String) -> FlusterResult<()> {
    tokio::fs::write(&fs_path, file_content)
        .await
        .map_err(|_| FlusterError::FailToSaveFile(fs_path.clone()))
}
