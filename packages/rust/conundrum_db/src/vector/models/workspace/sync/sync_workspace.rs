use conundrum::{
    ecosystem::{
        db::{db::ArcMutexDB, db_default_constants::DEFAULT_MAX_SYNC_THREADS},
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
    workspace::sync::{sync_context::SyncContext, sync_file_types::cdrm::sync_conundrum_path::sync_conundrum_path},
};

pub async fn sync_workspace(db: ArcMutexDB, walk_config: FileWalkConfig) -> DatabaseResult<SyncContext> {
    let max_threads = SettingsClient::max_sync_threads(Arc::clone(&db)).await?;
    let concurrency =
        Arc::new(Semaphore::new(match &max_threads {
                                    0 => std::thread::available_parallelism().map(|n| n.get())
                                                                             .unwrap_or(DEFAULT_MAX_SYNC_THREADS),
                                    _ => max_threads as usize,
                                }));
    let context = SyncContext::new(Arc::clone(&db)).await?;
    let ctx = Arc::new(tokio::sync::Mutex::new(context));
    let file_paths_arc = conundrum_fs::workspace_management::get_filetype_recursively::get_filetype_in_workspace_recursively(walk_config.clone()).await
            .map_err(|e| {
                log::error!("File System Error: {:#?}", e);
                DatabaseError::FileSystemError(e)
            })?;

    let file_paths_group = file_paths_arc.clone().lock_arc();

    let mut set = JoinSet::<DatabaseResult<ParsableFileType>>::new();

    for (pf, file_paths) in file_paths_group.clone() {
        for fp in file_paths {
            let db = Arc::clone(&db);
            let ctx = Arc::clone(&ctx);
            let pf = pf.clone();
            let root_path = walk_config.root.clone();
            let permits = Arc::clone(&concurrency);
            // let _permit = permits.acquire_owned().await.inspect_err(|e| {
            //                                                log::error!("Thread aquisition
            // error: {:?}", e);                                            });
            set.spawn(async move {
                   match pf {
                       ParsableFileType::Markdown | ParsableFileType::Cdrm | ParsableFileType::Mdx => {
                           log::debug!(
                                       "Parsing
    conundrum file at {}",
                                       fp.clone()
                    );
                           let ws_path =
    WorkspaceRelativePath::<PathBuf>::from_path_and_root(fp, root_path)
                               .map_err(|e| {
                                   DatabaseError::FileSystemError(e)
                               })?;
                           sync_conundrum_path(ws_path, Arc::clone(&db),
    Arc::clone(&ctx)).await.inspect_err(|e| {
    log::error!("Error: {:#?}", e);
    });
                           Ok(ParsableFileType::Cdrm)
                       }
                       _ => {
                           todo!()
                       }
                   }
               });
        }
    }

    while let Some(res) = set.join_next().await {
        if let Ok(parsed_file_type) = res {
            match parsed_file_type {
                Ok(unwrapped_file_type) => {
                    let mut context = ctx.clone().lock_owned().await;
                    context.count.increment_parsable_file_count(unwrapped_file_type);
                    drop(context);
                }
                Err(err) => {
                    log::error!("Error: {:#?}", err);
                }
            }
        } else {
            res.inspect_err(|e| {
                   log::error!("Threading Error: {:?}", e);
               });
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
