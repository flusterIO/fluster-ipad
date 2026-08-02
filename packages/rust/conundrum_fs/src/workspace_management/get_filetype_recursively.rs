use std::{
    fs,
    path::{self, Path},
    sync::Arc,
};

use conundrum::{
    ecosystem::error_handling::conundrum_fs_error::{ConundrumFSError, ConundrumFSResult},
    lang::constants::{file_names::CDRM_IGNORE_FILE_NAME, file_types::ParsableFileType},
};
use ignore::{
    types::{Types, TypesBuilder},
    WalkBuilder, WalkState,
};
use parking_lot::Mutex;
use pathdiff::diff_paths;

use crate::workspace_management::file_walk_config::FileWalkConfig;

pub fn get_types(ft: ParsableFileType) -> ConundrumFSResult<Types> {
    let mut types_builder = TypesBuilder::new();
    let (k, v) = ft.to_ignore_types();
    types_builder.add(k, v).map_err(|e| {
                                log::error!("Error: {:?}", e);
                                ConundrumFSError::GeneralFSError
                            })?;
    let r = types_builder.select(k).build().map_err(|e| {
                                                log::error!("Error: {:?}", e);
                                                ConundrumFSError::GeneralFSError
                                            })?;
    Ok(r)
}

/// Returns a list of *relative* paths matching the file extension.
pub async fn get_filetype_in_workspace_recursively(params: FileWalkConfig)
                                                   -> ConundrumFSResult<Arc<Mutex<Vec<String>>>> {
    let types = get_types(params.file_type)?;
    let file_paths: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let rp = Path::new(&params.root);
    let root_path = Arc::new(rp);
    WalkBuilder::new(params.root.clone()).git_ignore(params.respect_git_ignore)
                                         .hidden(params.ignore_hidden)
                                         .add_custom_ignore_filename(CDRM_IGNORE_FILE_NAME)
                                         .types(types)
                                         .build_parallel()
                                         .run(|| {
                                             let fp = Arc::clone(&file_paths);
                                             let root = Arc::clone(&root_path);
                                             Box::new(move |res| {
                                                 if let Ok(entry) = res {
                                                     if entry.file_type().map_or(false, |f| f.is_file()) {
                                                         let f = entry.path();
                                                         println!("F: {:?}", f);
                                                         if let Some(p) = diff_paths(f, *root) {
                                                             println!("Path: {:?}", p);
                                                             if let Some(substring) = p.to_str() {
                                                                 let mut _file_paths = fp.clone().lock_arc();
                                                                 _file_paths.push(substring.to_string());
                                                                 drop(_file_paths);
                                                             }
                                                         }
                                                     }
                                                 }
                                                 WalkState::Continue
                                             })
                                         });
    Ok(file_paths)
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[tokio::test]
    async fn returns_file_types() {
        for ft in ParsableFileType::iter() {
            let r  = get_filetype_in_workspace_recursively(FileWalkConfig { root: "/Users/bigsexy/Desktop/notes/content/".to_string(), respect_git_ignore: true, ignore_hidden: true, file_type: ft }).await.expect("gets file types without throwing an error.");
            let _r = r.clone().lock_arc();
            assert!(!_r.is_empty(), "File types found is not empty");
        }
        // assert_eq!(result, 4);
    }
}
