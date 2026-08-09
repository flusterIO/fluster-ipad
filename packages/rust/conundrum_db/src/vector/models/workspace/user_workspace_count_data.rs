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
    /// The total number of each parsable file found.
    pub count: HashMap<ParsableFileType, u32>,
}

impl TryFromAsync<UserWorkspace> for UserWorkspaceCountData {
    async fn try_from_async(input: UserWorkspace) -> DatabaseResult<UserWorkspaceCountData> {
        let count = get_workspace_count(FileCountConfig { root: input.root.clone(),
                                                          respect_gitignore: input.respect_gitignore,
                                                          ignore_hidden: input.ignore_hidden }).await?;
        Ok(UserWorkspaceCountData { workspace: input.clone(),
                                    count })
    }
}
