use conundrum::{
    ecosystem::{
        db::{db_client::db_client::DBClient, db_default_constants::DEFAULT_MAX_SYNC_THREADS},
        error_handling::db_error::{DatabaseError, DatabaseResult},
    },
    lang::constants::file_types::ParsableFileType,
};
use conundrum_fs::{
    models::user_workspace::workspace_relative_path::WorkspaceRelativePath,
    workspace_management::file_walk_config::FileWalkConfig,
};
use std::{path::PathBuf, sync::Arc};
use tokio::{sync::Semaphore, task::JoinSet};

use crate::vector::models::{
    ecosystem_data::ecosystem_setting_key::settings_client::SettingsClient,
    workspace::{
        sync::{sync_context::SyncContext, sync_file_types::cdrm::sync_conundrum_path::sync_conundrum_path},
        user_workspace::UserWorkspace,
    },
};

pub async fn sync_registered_workspace(db: DBClient, workspace: UserWorkspace) -> DatabaseResult<SyncContext> {
    let walk_config = FileWalkConfig { root: workspace.root,
                                       respect_git_ignore: workspace.respect_gitignore,
                                       ignore_hidden: workspace.ignore_hidden };
    sync_workspace(db, walk_config).await
}

pub async fn sync_workspace(db: DBClient, walk_config: FileWalkConfig) -> DatabaseResult<SyncContext> {
    let max_threads = SettingsClient::max_sync_threads(db.clone()).await?;
    let concurrency =
        Arc::new(Semaphore::new(match &max_threads {
                                    0 => {
                                        std::thread::available_parallelism().map(|n| n.get())
                                                                            .unwrap_or(DEFAULT_MAX_SYNC_THREADS
                                                                                       as usize)
                                    }
                                    _ => max_threads as usize,
                                }));
    let context = SyncContext::new(db.clone()).await?;
    let ctx = Arc::new(tokio::sync::Mutex::new(context));
    let file_paths_arc = conundrum_fs::workspace_management::get_filetype_recursively::get_filetype_in_workspace_recursively(walk_config.clone()).await
            .map_err(|e| {
                log::error!("File System Error: {:#?}", e);
                DatabaseError::FileSystemError(e)
            })?;

    let file_paths_group = {
        let file_paths = file_paths_arc.clone().lock_arc();
        file_paths.clone()
    };

    let mut set = JoinSet::<DatabaseResult<ParsableFileType>>::new();

    for (pf, file_paths) in file_paths_group {
        if !matches!(pf, ParsableFileType::Markdown | ParsableFileType::Cdrm | ParsableFileType::Mdx) {
            continue;
        }
        for fp in file_paths {
            let db = db.clone();
            let ctx = Arc::clone(&ctx);
            let pf = pf.clone();
            let root_path = walk_config.root.clone();
            let permits = Arc::clone(&concurrency);
            set.spawn(async move {
                   let _permit = permits.acquire_owned().await.map_err(|_| DatabaseError::ThreadError)?;
                   log::debug!("Parsing Conundrum file at {}", fp);
                   let ws_path = WorkspaceRelativePath::<PathBuf>::from_path_and_root(fp, root_path)
                       .map_err(DatabaseError::FileSystemError)?;
                   sync_conundrum_path(ws_path, db, Arc::clone(&ctx)).await?;
                   Ok(pf)
               });
        }
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(Ok(file_type)) => {
                let mut context = ctx.clone().lock_owned().await;
                context.count.increment_parsable_file_count(file_type);
            }
            Ok(Err(err)) => log::error!("Failed to synchronize a Conundrum file: {err:#?}"),
            Err(err) => log::error!("Conundrum synchronization task failed: {err:?}"),
        }
    }

    match Arc::try_unwrap(ctx) {
        Ok(m) => Ok(m.into_inner()),
        Err(_err) => {
            println!("Attempted to consume arc while references still exist.");
            let x = _err.clone().lock_owned().await;
            let y = x.clone();
            Ok(y)
        }
    }
}
