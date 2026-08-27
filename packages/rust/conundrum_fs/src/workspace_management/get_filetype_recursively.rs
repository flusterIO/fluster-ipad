use std::{
    collections::HashMap,
    path::{self, Path},
    str::FromStr,
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
use strum::IntoEnumIterator;

use crate::workspace_management::file_walk_config::FileWalkConfig;

pub fn get_types() -> ConundrumFSResult<Types> {
    let mut types_builder = TypesBuilder::new();
    for ft in ParsableFileType::iter() {
        let (k, v) = ft.to_ignore_types();
        types_builder.add(k, v).map_err(|e| {
                                    log::error!("Error: {:?}", e);
                                    ConundrumFSError::GeneralFSError
                                })?;
    }
    let r = types_builder.select("all").build().map_err(|e| {
                                                    log::error!("Error: {:?}", e);
                                                    ConundrumFSError::GeneralFSError
                                                })?;
    Ok(r)
}

pub type ParsableFileTypePathMap = Arc<Mutex<HashMap<ParsableFileType, Vec<String>>>>;

/// Returns a list of *absolute* paths matching the file extension.
pub async fn get_filetype_in_workspace_recursively(params: FileWalkConfig)
                                                   -> ConundrumFSResult<ParsableFileTypePathMap> {
    let types = get_types()?;
    let file_paths: Arc<Mutex<HashMap<ParsableFileType, Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));
    let rp = Path::new(&params.root);
    let root_path = Arc::new(rp);
    WalkBuilder::new(params.root.clone()).git_ignore(params.respect_git_ignore)
                                         .hidden(params.ignore_hidden)
                                         .add_custom_ignore_filename(CDRM_IGNORE_FILE_NAME)
                                         .types(types)
                                         .build_parallel()
                                         .run(|| {
                                             let fp = Arc::clone(&file_paths);
                                             Box::new(move |res| {
                                                 if let Ok(entry) = res {
                                                     let entry_path = entry.path();
                                                         if let Some(file_extension) = entry_path.extension() {
                                                             if let Some(file_ext_str) = file_extension.to_str() {
                                                             if let Ok(pf) = ParsableFileType::from_str(file_ext_str).map_err(|e| {
                                                                         ConundrumFSError::UnsupportedFileExtension(format!("{:?}", file_extension))
                                                                     }) {
                                                                 let mut paths = fp.clone().lock_arc();
                                                                 if let Some(existing) = paths.get_mut(&pf) {
                                                                     if let Some(p) = entry_path.to_str() {
                                                                     existing.push(p.to_string());
                                                                     } else {
                                                                         log::error!("Failed to parse file path to a string.")
                                                                     }
                                                                 } else {
                                                                     if let Some(p) = entry_path.to_str() {
                                                                         paths.insert(pf.clone(), vec![p.to_string()]);
                                                                     } else {
                                                                         log::error!("Failed to parse file path to a string.")
                                                                     }
                                                                 }
                                                                 drop(paths);
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

    #[test_log::test(tokio::test)]
    async fn returns_file_types() {
        for ft in ParsableFileType::iter() {
            let r  = get_filetype_in_workspace_recursively(FileWalkConfig { root: "/Users/bigsexy/Desktop/notes/content/".to_string(), respect_git_ignore: true, ignore_hidden: true }).await.expect("gets file types without throwing an error.");
            let _r = r.clone().lock_arc();
            println!("{:#?}", _r.clone());
            assert!(!_r.is_empty(), "File types found is not empty");
        }
        // assert_eq!(result, 4);
    }
}
