use std::{path::PathBuf, sync::Arc};

use conundrum::{
    ecosystem::{
        db::{
            db_client::db_client::DBClient,
            db_traits::{
                db_entity::DBSchema,
                entity_crud::{EntityCRUD, filter_one},
            },
        },
        error_handling::db_error::{DatabaseError, DatabaseResult},
    },
    lang::{
        lib::{shared::utility_types::ArcTokioMutex, std_lib_impls::json_string::QuotedString},
        runtime::run_conundrum::{ParseConundrumOptions, run_conundrum},
    },
};
use conundrum_fs::models::user_workspace::workspace_relative_path::WorkspaceRelativePath;

use crate::vector::models::{
    text::cdrm::cdrm_model::CdrmModel,
    workspace::sync::{
        sync_context::SyncContext,
        sync_file_types::cdrm::update_database_from_parsed_cdrm::update_database_from_parsed_cdrm,
    },
};

pub async fn sync_conundrum_path<'a>(fp: WorkspaceRelativePath<PathBuf>,
                                     database: DBClient,
                                     ctx: ArcTokioMutex<SyncContext>)
                                     -> DatabaseResult<()> {
    let workspace_path = fp.workspace_path.to_str().ok_or(DatabaseError::SerializationError)?.to_string();
    let relative_path = fp.relative_path.to_str().ok_or(DatabaseError::SerializationError)?.to_string();
    let existing_notes =
        <CdrmModel as EntityCRUD< <CdrmModel as DBSchema>::PartialUpdateType>>::get_by_predicate(Some(format!("ws_root = {} AND relative_path = {}",
                                                                 workspace_path.to_quoted_string()?,
                                                                 relative_path.to_quoted_string()?)),
                                                    None,
                                                    None,
                                                    database.clone()).await?;
    let existing_note = filter_one(existing_notes)?;
    let existing_content = existing_note.clone().map(|x| x.0.content.0.clone());
    let file_content = tokio::fs::read_to_string(fp.absolutize()).await
        .map_err(|e| {
           log::error!("Error: {:#?}", e);
           DatabaseError::FileSystemError(conundrum::ecosystem::error_handling::conundrum_fs_error::ConundrumFSError::FsError(e.to_string()))
        })?;
    if existing_content.clone().is_some_and(|s| s == file_content) {
        log::debug!(
                    "Bypassing an already valid file base on a file content match
    at {:?}",
                    fp.absolutize().display()
        );
        return Ok(());
    }
    drop(existing_content);
    let existing_note_id = existing_note.clone().map(|x| x.0.id.clone());
    let opts = ParseConundrumOptions::for_syncing_ecosystem_database(file_content.as_str(), existing_note_id);
    let parsed = run_conundrum(opts).map_err(|e| {
                                        log::error!(
                                                    "Conundrum encountered the following error while parsing
    Conundrum content at {}. \n{:#?}",
                                                    fp.absolutize().display(),
                                                    e
        );
                                        DatabaseError::ConundrumError(e)
                                    })?;

    update_database_from_parsed_cdrm(parsed.clone(),
                                     existing_note.clone(),
                                     file_content.clone(),
                                     workspace_path.clone(),
                                     relative_path.clone(),
                                     database.clone(),
                                     Arc::clone(&ctx)).await?;
    Ok(())
}
