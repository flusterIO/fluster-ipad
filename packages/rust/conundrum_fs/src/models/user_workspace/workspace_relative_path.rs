use std::path::PathBuf;

use fake::faker::filesystem::en::{DirPath, FilePath};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use surrealdb_types::{Object, SurrealValue};

use crate::{errors::conundrum_fs_error::ConundrumFSResult, models::user_workspace::cdrm_path_buf::CDRMPathBuf};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkspaceRelativePath {
    pub workspace_path: CDRMPathBuf,
    pub relative_path: CDRMPathBuf,
}

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue, Dummy)]
pub struct PureWorkspaceRelativePath {
    #[dummy(faker = "DirPath()")]
    pub workspace_path: String,
    #[dummy(faker = "FilePath()")]
    pub relative_path: String,
}

impl WorkspaceRelativePath {
    pub fn ty_to_pure_model(&self) -> ConundrumFSResult<PureWorkspaceRelativePath> {
        Ok(PureWorkspaceRelativePath { workspace_path: self.workspace_path.try_to_string()?,
                                       relative_path: self.relative_path.try_to_string()? })
    }
}

impl PureWorkspaceRelativePath {
    pub fn to_workspace_relative_path(&self) -> ConundrumFSResult<WorkspaceRelativePath> {
        Ok(WorkspaceRelativePath { workspace_path:
                                       CDRMPathBuf::from_pathbuf(PathBuf::new().join(self.workspace_path.clone())),
                                   relative_path: CDRMPathBuf::from_pathbuf(PathBuf::new().join(self.relative_path
                                                                                                    .clone())) })
    }
}
