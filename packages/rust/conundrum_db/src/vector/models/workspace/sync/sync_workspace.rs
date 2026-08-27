use conundrum::{
    ecosystem::{
        db::db::ArcMutexDB,
        error_handling::db_error::{DatabaseError, DatabaseResult},
    },
    lang::constants::file_types::ParsableFileType,
};
use conundrum_fs::workspace_management::file_walk_config::FileWalkConfig;
use std::sync::Arc;
use tokio::task::JoinSet;

use crate::vector::models::workspace::sync::{
    sync_context::SyncContext, sync_file_types::sync_conundrum_path::sync_conundrum_path,
};

pub async fn sync_workspace(db: ArcMutexDB, walk_config: FileWalkConfig) -> DatabaseResult<SyncContext> {
    let ctx = Arc::new(tokio::sync::Mutex::new(SyncContext::default()));
    let file_paths_arc = conundrum_fs::workspace_management::get_filetype_recursively::get_filetype_in_workspace_recursively(walk_config).await
            .map_err(|e| {
                log::error!("File System Error: {:#?}", e);
                DatabaseError::FileSystemError(e)
            })?;

    let file_paths_group = file_paths_arc.clone().lock_arc();

    let mut set = JoinSet::<DatabaseResult<ParsableFileType>>::new();

    for (pf, file_paths) in file_paths_group.clone() {
        for fp in file_paths {
            let db = Arc::clone(&db);
            let pf = pf.clone();
            set.spawn(async move {
                   match pf {
                       ParsableFileType::Markdown | ParsableFileType::Cdrm | ParsableFileType::Mdx => {
                           log::debug!("Parsing conundrum file at {}", fp.clone());
                           sync_conundrum_path(fp.clone(), &Arc::clone(&db)).await.inspect_err(|e| {
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
                    context.increment_parsable_file_count(unwrapped_file_type);
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
