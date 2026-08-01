use std::path::PathBuf;

use conundrum::ecosystem::db::tables::DatabaseTable;
use fake::faker::filesystem::en::{DirPath, FilePath};
use fake::Dummy;
use indoc::formatdoc;
use serde::{Deserialize, Serialize};

use crate::{errors::conundrum_fs_error::ConundrumFSResult, models::user_workspace::cdrm_path_buf::CDRMPathBuf};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkspaceRelativePath<T = CDRMPathBuf> {
    pub workspace_path: T,
    pub relative_path: T,
}
