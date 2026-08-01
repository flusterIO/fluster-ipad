use serde::{Deserialize, Serialize};

use crate::models::user_workspace::cdrm_path_buf::CDRMPathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkspaceRelativePath<T = CDRMPathBuf> {
    pub workspace_path: T,
    pub relative_path: T,
}
