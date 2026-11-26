//! # File system sync
//!
//! A small REST API that lets native clients (currently the Fluster desktop
//! app) mirror a user's notes directory into their own local database.
//!
//! The server owns *file system access only*: it walks a directory, reports a
//! manifest of note files (path, timestamps, content hash and front-matter
//! identity) and serves file contents on demand. The client owns the
//! decision of what to create, update or remove in its database. This keeps
//! the sandboxed desktop app free of broad file system entitlements and keeps
//! the protocol reusable for other clients.
//!
//! The protocol is two-phase so large directories stay cheap to re-sync:
//!
//! 1. `POST /api/v1/fs_sync/manifest` → every note file with its SHA-256 hash.
//! 2. `POST /api/v1/fs_sync/contents` → the contents of only the files the
//!    client found to be new or changed.
//!
//! Bump [`models::FS_SYNC_PROTOCOL_VERSION`] whenever a response shape
//! changes in a way older clients can't decode.
pub mod error;
pub mod handlers;
pub mod models;
pub mod note_file;
pub mod note_file_kind;
pub mod path_scope;
pub mod router;
pub mod scanner;
