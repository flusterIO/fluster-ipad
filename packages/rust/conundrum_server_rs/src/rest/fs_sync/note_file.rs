//! Reading a single note file into the shape both endpoints need.
use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use conundrum_lang::output::parsing_result::mdx_parsing_result::MdxParsingResult;
use sha2::{Digest, Sha256};

use crate::rest::fs_sync::models::{FileIssueReason, ManifestFrontMatter};

/// Files larger than this are skipped. Notes are text; anything this large is
/// almost certainly not something a user wants parsed on every sync.
pub const MAX_NOTE_FILE_BYTES: u64 = 8 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct NoteFile {
    pub content: String,
    pub content_hash: String,
    pub size_bytes: u64,
    pub modified_at_ms: i64,
    pub created_at_ms: Option<i64>,
}

impl NoteFile {
    /// Reads a file that has already been confined to a [`super::path_scope::SyncRoot`].
    pub fn read(path: &Path) -> Result<NoteFile, FileIssueReason> {
        let metadata = std::fs::metadata(path).map_err(io_reason)?;
        if !metadata.is_file() {
            return Err(FileIssueReason::NotAFile);
        }
        if metadata.len() > MAX_NOTE_FILE_BYTES {
            return Err(FileIssueReason::TooLarge);
        }
        let bytes = std::fs::read(path).map_err(io_reason)?;
        let content_hash = hash_bytes(&bytes);
        let size_bytes = bytes.len() as u64;
        let content = String::from_utf8(bytes).map_err(|_| FileIssueReason::NotUtf8)?;
        Ok(NoteFile { content,
                      content_hash,
                      size_bytes,
                      modified_at_ms: metadata.modified().map(system_time_to_ms).unwrap_or(0),
                      created_at_ms: metadata.created().ok().map(system_time_to_ms) })
    }

    /// Extracts identity fields using Conundrum's own front matter parser.
    pub fn front_matter(&self) -> ManifestFrontMatter {
        match MdxParsingResult::from_initial_mdx_content(&self.content).front_matter {
            Some(fm) => ManifestFrontMatter { user_defined_id: fm.user_defined_id,
                                              title: fm.title },
            None => ManifestFrontMatter::default(),
        }
    }
}

/// Lowercase hex SHA-256. Clients must hash the UTF-8 bytes of their stored
/// note body the same way to detect changes.
pub fn hash_bytes(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn system_time_to_ms(time: SystemTime) -> i64 {
    match time.duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_millis() as i64,
        Err(e) => -(e.duration().as_millis() as i64),
    }
}

fn io_reason(error: std::io::Error) -> FileIssueReason {
    match error.kind() {
        std::io::ErrorKind::NotFound => FileIssueReason::NotFound,
        _ => FileIssueReason::Unreadable,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::fs_sync::path_scope::test_support::TempDir;

    #[test]
    fn hashes_match_the_sha256_reference_vector() {
        assert_eq!(hash_bytes(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }

    #[test]
    fn reads_content_and_front_matter() {
        let dir = TempDir::new("note_file");
        let path = dir.write("note.cdrm", "---\nid: my-note\ntitle: My Note\n---\n# Body\n");
        let file = NoteFile::read(&path).unwrap();
        assert_eq!(file.size_bytes, file.content.len() as u64);
        assert_eq!(file.content_hash, hash_bytes(file.content.as_bytes()));
        let fm = file.front_matter();
        assert_eq!(fm.user_defined_id.as_deref(), Some("my-note"));
        assert_eq!(fm.title.as_deref(), Some("My Note"));
    }

    #[test]
    fn rejects_non_utf8_files() {
        let dir = TempDir::new("not_utf8");
        let path = dir.path().join("bad.md");
        std::fs::write(&path, [0xff, 0xfe, 0xfd]).unwrap();
        assert_eq!(NoteFile::read(&path).unwrap_err(), FileIssueReason::NotUtf8);
    }
}
