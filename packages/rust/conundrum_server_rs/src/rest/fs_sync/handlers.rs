//! Thin axum handlers: translate the request, run blocking file work off the
//! async executor, and map the result to a response.
use axum::Json;

use crate::rest::fs_sync::{
    error::FsSyncError,
    models::{
        ContentsRequest, ContentsResponse, FS_SYNC_PROTOCOL_VERSION, FileContent, FileIssue, FileIssueReason,
        MAX_CONTENT_PATHS_PER_REQUEST, ManifestRequest, ManifestResponse,
    },
    note_file::NoteFile,
    note_file_kind::NoteFileKind,
    path_scope::SyncRoot,
    scanner::{self, ScanOptions},
};

async fn run_blocking<T, F>(work: F) -> Result<T, FsSyncError>
    where F: FnOnce() -> Result<T, FsSyncError> + Send + 'static,
          T: Send + 'static
{
    tokio::task::spawn_blocking(work).await.map_err(|e| FsSyncError::Internal(e.to_string()))?
}

pub async fn manifest(Json(request): Json<ManifestRequest>) -> Result<Json<ManifestResponse>, FsSyncError> {
    run_blocking(move || {
        let root = SyncRoot::open(&request.root)?;
        let options = ScanOptions::from(&request);
        Ok(scanner::scan(&root, &options))
    }).await
      .map(Json)
}

pub async fn contents(Json(request): Json<ContentsRequest>) -> Result<Json<ContentsResponse>, FsSyncError> {
    if request.relative_paths.len() > MAX_CONTENT_PATHS_PER_REQUEST {
        return Err(FsSyncError::TooManyPaths { requested: request.relative_paths.len(),
                                               max: MAX_CONTENT_PATHS_PER_REQUEST });
    }
    run_blocking(move || {
        let root = SyncRoot::open(&request.root)?;
        let mut files: Vec<FileContent> = Vec::with_capacity(request.relative_paths.len());
        let mut failures: Vec<FileIssue> = Vec::new();
        for relative_path in request.relative_paths {
            match read_confined(&root, &relative_path) {
                Ok(file) => files.push(FileContent { relative_path,
                                                     content: file.content,
                                                     content_hash: file.content_hash,
                                                     modified_at_ms: file.modified_at_ms,
                                                     created_at_ms: file.created_at_ms }),
                Err(reason) => failures.push(FileIssue { relative_path, reason }),
            }
        }
        Ok(ContentsResponse { protocol_version: FS_SYNC_PROTOCOL_VERSION,
                              files,
                              failures })
    }).await
      .map(Json)
}

/// Only note files inside the root may be read through this API.
fn read_confined(root: &SyncRoot, relative_path: &str) -> Result<NoteFile, FileIssueReason> {
    let resolved = root.resolve(relative_path)?;
    if NoteFileKind::from_path(&resolved).is_none() {
        return Err(FileIssueReason::UnsupportedFileKind);
    }
    NoteFile::read(&resolved)
}
