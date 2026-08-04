use std::collections::HashMap;

use conundrum::{ecosystem::error_handling::db_error::DatabaseResult, lang::constants::file_types::ParsableFileType};
use conundrum_fs::workspace_management::{file_walk_config::FileCountConfig, get_workspace_count::get_workspace_count};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::vector::{
    database::db_traits::async_traits::try_from_async::TryFromAsync, models::workspace::user_workspace::UserWorkspace,
};

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct UserWorkspaceCountData {
    pub workspace: UserWorkspace,
    pub count: HashMap<ParsableFileType, u32>,
    /// Returns a map of all bib paths provided by the user, and the default bib
    /// path and a boolean indicating whether they exist or not.
    pub bib_path_exists: HashMap<String, bool>,
}

impl TryFromAsync<UserWorkspace> for UserWorkspaceCountData {
    async fn try_from_async(input: UserWorkspace) -> DatabaseResult<UserWorkspaceCountData> {
        let bib_path_exists = input.valid_bib_paths().await?;
        let count = get_workspace_count(FileCountConfig { root: input.root.clone(),
                                                          respect_gitignore: input.respect_gitignore,
                                                          ignore_hidden: input.ignore_hidden }).await?;
        Ok(UserWorkspaceCountData { workspace: input.clone(),
                                    count,
                                    bib_path_exists })
    }
}
