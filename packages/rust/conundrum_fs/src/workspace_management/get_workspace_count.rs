use conundrum::{
    ecosystem::error_handling::conundrum_fs_error::{ConundrumFSError, ConundrumFSResult},
    lang::constants::{file_names::CDRM_IGNORE_FILE_NAME, file_types::ParsableFileType},
};
use ignore::{WalkBuilder, WalkState};
use parking_lot::Mutex;
use std::{collections::HashMap, str::FromStr, sync::Arc};
use strum::IntoEnumIterator;

use crate::workspace_management::{
    file_walk_all_types::get_all_parsable_ignore_types, file_walk_config::FileCountConfig,
};

/// NOTE: This will fallback to cloning if a reference still exists. That may
/// not be the best behavior in all circumstances.
pub fn consume_arc_mutex<T: Clone>(arc_mutex: Arc<Mutex<T>>) -> ConundrumFSResult<T> {
    match Arc::try_unwrap(arc_mutex) {
        Ok(m) => Ok(m.into_inner()),
        Err(_err) => {
            println!("Attempted to consume arc while references still exist.");
            let x = _err.lock_arc();
            let y = x.clone();
            Ok(y)
        }
    }
}

pub async fn get_workspace_count(params: FileCountConfig) -> ConundrumFSResult<HashMap<ParsableFileType, u32>> {
    let mut hm: HashMap<ParsableFileType, u32> = HashMap::new();
    for pf in ParsableFileType::iter() {
        hm.insert(pf, 0);
    }
    let data: Arc<Mutex<HashMap<ParsableFileType, u32>>> = Arc::new(Mutex::new(hm));
    let types = get_all_parsable_ignore_types()?;
    WalkBuilder::new(params.root.clone()).git_ignore(params.respect_gitignore)
                                         .hidden(params.ignore_hidden)
                                         .add_custom_ignore_filename(CDRM_IGNORE_FILE_NAME)
                                         .types(types)
                                         .build_parallel()
                                         .run(|| {
                                             let data = Arc::clone(&data);
                                             Box::new(move |res| {
                                                 if let Ok(entry) = res {
                                                     if entry.file_type().map_or(false, |f| f.is_file()) {
                                                         if let Some(file_extension) = entry.path().extension() {
                                                             if let Some(file_ext_str) = file_extension.to_str() {
                                                             if let Ok(pf) = ParsableFileType::from_str(file_ext_str).map_err(|e| {
                                                                         println!("Error: {:?}", e);
                                                                         log::error!("Error: {:?}", e);
                                                                         ConundrumFSError::UnsupportedFileExtension(format!("{:?}", file_extension))
                                                                     }) {
                                                                 let mut hm = data.clone().lock_arc();
                                                                 let current_value = &hm.get_mut(&pf).map(|n| *n).unwrap_or(0);
                                                                 hm.insert(pf, *current_value + 1);
                                                                 drop(hm);
                                                             }
                                                         }
                                                             }
                                                     }
                                                 }
                                                 WalkState::Continue
                                             })
                                         });
    let final_data = consume_arc_mutex(data)?;
    Ok(final_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn gets_workspace_count() {
        let res =
            get_workspace_count(FileCountConfig { root: "/Users/bigsexy/Desktop/notes/content/".to_string(),
                                                  respect_gitignore: true,
                                                  ignore_hidden: true }).await
                                                                        .inspect_err(|e| {
                                                                            println!("Error: {:?}", e);
                                                                        })
                                                                        .expect("Generates a parsable file count.");
        println!("Output: {:#?}", res);
        // assert_eq!(result, 4);
    }
}
