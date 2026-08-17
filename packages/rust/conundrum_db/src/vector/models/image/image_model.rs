use conundrum::lifted_models::primitives::bytes::Bytes;
use conundrum_fs::models::user_workspace::{
    workspace_relative_path::WorkspaceRelativePath, workspace_relative_path_strings::WorkspaceRelativeStringPath,
};

use crate::vector::models::taggables::taggables::Taggables;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct ImageModel {
    /// An optional id user's can use to reference this image in their notes. A
    /// similar feature was in the initial version of Fluster, but this
    /// feature has yet to make the migration to Conundrum.
    pub user_defined_id: Option<String>,
    pub path: Option<WorkspaceRelativeStringPath>,
    pub data: Bytes,
    pub taggables: Taggables,
}
