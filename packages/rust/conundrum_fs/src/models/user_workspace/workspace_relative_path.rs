use std::path::PathBuf;

use conundrum::ecosystem::error_handling::conundrum_fs_error::ConundrumFSResult;
use serde::{Deserialize, Serialize};

use crate::models::user_workspace::cdrm_path_buf::CDRMPathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkspaceRelativePath<T = CDRMPathBuf> {
    pub workspace_path: T,
    pub relative_path: T,
}

impl WorkspaceRelativePath<PathBuf> {
    pub fn absolutize(&self) -> PathBuf {
        self.workspace_path.join(self.relative_path.clone())
    }

    pub fn from_path_and_root(absolute_path: String, root: String) -> ConundrumFSResult<Self> {
        if let Some(relative_path) = pathdiff::diff_paths(absolute_path, root.clone()) {
            Ok(WorkspaceRelativePath { workspace_path: PathBuf::new().join(root),
                                       relative_path })
        } else {
            log::warn!("Attempted to get a nested path from a parent that isn't that path's parent.");
            Err(conundrum::ecosystem::error_handling::conundrum_fs_error::ConundrumFSError::PathSerializationError)
        }
    }
}
