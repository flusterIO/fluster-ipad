//! Wire types for the fs_sync API. Field names are snake_case on the wire.
use serde::{Deserialize, Serialize};

use crate::rest::fs_sync::note_file_kind::NoteFileKind;

/// Incremented whenever a response shape changes incompatibly. Clients
/// compare this against the version they were built for.
pub const FS_SYNC_PROTOCOL_VERSION: u32 = 1;

/// Upper bound on the number of files a single `contents` request may ask
/// for. Clients batch requests below this.
pub const MAX_CONTENT_PATHS_PER_REQUEST: usize = 500;

fn default_true() -> bool {
    true
}

#[derive(Deserialize, Debug, Clone)]
pub struct ManifestRequest {
    /// Absolute path of the notes directory.
    pub root: String,
    #[serde(default = "default_true")]
    pub respect_gitignore: bool,
    #[serde(default)]
    pub include_hidden: bool,
    /// Kinds of files to include. Defaults to every [`NoteFileKind`].
    #[serde(default)]
    pub file_kinds: Option<Vec<NoteFileKind>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ManifestResponse {
    pub protocol_version: u32,
    /// The canonicalized root that every `relative_path` is relative to.
    pub root: String,
    pub generated_at_ms: i64,
    pub entries: Vec<ManifestEntry>,
    /// Note files that matched but could not be included.
    pub skipped: Vec<FileIssue>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ManifestEntry {
    /// `/`-separated path relative to the root. This is the file's identity.
    pub relative_path: String,
    pub file_kind: NoteFileKind,
    pub size_bytes: u64,
    pub modified_at_ms: i64,
    pub created_at_ms: Option<i64>,
    /// Lowercase hex SHA-256 of the raw file bytes.
    pub content_hash: String,
    pub front_matter: ManifestFrontMatter,
}

/// The parts of a note's front matter a client needs to match a file to an
/// existing record before downloading its contents.
#[derive(Serialize, Debug, Clone, Default)]
pub struct ManifestFrontMatter {
    pub user_defined_id: Option<String>,
    pub title: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContentsRequest {
    pub root: String,
    pub relative_paths: Vec<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ContentsResponse {
    pub protocol_version: u32,
    pub files: Vec<FileContent>,
    pub failures: Vec<FileIssue>,
}

#[derive(Serialize, Debug, Clone)]
pub struct FileContent {
    pub relative_path: String,
    pub content: String,
    pub content_hash: String,
    pub modified_at_ms: i64,
    pub created_at_ms: Option<i64>,
}

#[derive(Serialize, Debug, Clone)]
pub struct FileIssue {
    pub relative_path: String,
    pub reason: FileIssueReason,
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FileIssueReason {
    TooLarge,
    NotUtf8,
    Unreadable,
    NotFound,
    NotAFile,
    OutsideRoot,
    UnsupportedFileKind,
}

#[derive(Serialize, Debug, Clone)]
pub struct HealthResponse {
    pub status: String,
    pub server_version: String,
    pub fs_sync_protocol_version: u32,
}
