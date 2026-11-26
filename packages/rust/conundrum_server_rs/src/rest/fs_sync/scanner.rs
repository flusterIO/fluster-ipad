//! Walks a sync root and builds the manifest.
use ignore::WalkBuilder;

use crate::rest::fs_sync::{
    models::{FS_SYNC_PROTOCOL_VERSION, FileIssue, ManifestEntry, ManifestRequest, ManifestResponse},
    note_file::{NoteFile, system_time_to_ms},
    note_file_kind::NoteFileKind,
    path_scope::SyncRoot,
};

#[derive(Debug, Clone)]
pub struct ScanOptions {
    pub respect_gitignore: bool,
    pub include_hidden: bool,
    pub file_kinds: Vec<NoteFileKind>,
}

impl From<&ManifestRequest> for ScanOptions {
    fn from(request: &ManifestRequest) -> ScanOptions {
        ScanOptions { respect_gitignore: request.respect_gitignore,
                      include_hidden: request.include_hidden,
                      file_kinds: request.file_kinds.clone().unwrap_or_else(NoteFileKind::all) }
    }
}

/// Blocking. Call from `spawn_blocking`.
pub fn scan(root: &SyncRoot, options: &ScanOptions) -> ManifestResponse {
    let mut builder = WalkBuilder::new(root.path());
    builder.hidden(!options.include_hidden)
           .git_ignore(options.respect_gitignore)
           .git_global(options.respect_gitignore)
           .git_exclude(options.respect_gitignore)
           .ignore(options.respect_gitignore)
           .parents(options.respect_gitignore)
           // Notes directories often aren't git repositories, but users still
           // expect a `.gitignore` in them to be honored.
           .require_git(false)
           .follow_links(false);

    let mut entries: Vec<ManifestEntry> = Vec::new();
    let mut skipped: Vec<FileIssue> = Vec::new();

    for result in builder.build() {
        let dir_entry = match result {
            Ok(e) => e,
            Err(e) => {
                log::warn!("fs_sync: skipping an unreadable directory entry: {}", e);
                continue;
            }
        };
        if !dir_entry.file_type().is_some_and(|t| t.is_file()) {
            continue;
        }
        let path = dir_entry.path();
        let Some(file_kind) = NoteFileKind::from_path(path) else {
            continue;
        };
        if !options.file_kinds.contains(&file_kind) {
            continue;
        }
        let Some(relative_path) = root.relativize(path) else {
            log::warn!("fs_sync: skipping a path that is not valid UTF-8 or not inside the root.");
            continue;
        };
        match NoteFile::read(path) {
            Ok(file) => entries.push(ManifestEntry { front_matter: file.front_matter(),
                                                     relative_path,
                                                     file_kind,
                                                     size_bytes: file.size_bytes,
                                                     modified_at_ms: file.modified_at_ms,
                                                     created_at_ms: file.created_at_ms,
                                                     content_hash: file.content_hash }),
            Err(reason) => skipped.push(FileIssue { relative_path, reason }),
        }
    }

    entries.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    log::debug!("fs_sync: manifest built with {} entries and {} skipped files.", entries.len(), skipped.len());

    ManifestResponse { protocol_version: FS_SYNC_PROTOCOL_VERSION,
                       root: root.display(),
                       generated_at_ms: system_time_to_ms(std::time::SystemTime::now()),
                       entries,
                       skipped }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::fs_sync::path_scope::test_support::TempDir;

    fn options() -> ScanOptions {
        ScanOptions { respect_gitignore: true,
                      include_hidden: false,
                      file_kinds: NoteFileKind::all() }
    }

    fn paths(response: &ManifestResponse) -> Vec<&str> {
        response.entries.iter().map(|e| e.relative_path.as_str()).collect()
    }

    #[test]
    fn finds_note_files_and_ignores_everything_else() {
        let dir = TempDir::new("scan_basic");
        dir.write("a.cdrm", "# A");
        dir.write("nested/b.md", "# B");
        dir.write("nested/deeper/c.mdx", "# C");
        dir.write("image.png", "not a note");
        dir.write("readme.txt", "not a note");
        let root = SyncRoot::open(dir.path().to_str().unwrap()).unwrap();
        let response = scan(&root, &options());
        assert_eq!(paths(&response), vec!["a.cdrm", "nested/b.md", "nested/deeper/c.mdx"]);
        assert!(response.skipped.is_empty());
        assert_eq!(response.protocol_version, FS_SYNC_PROTOCOL_VERSION);
    }

    #[test]
    fn respects_gitignore_and_hidden_files_when_asked() {
        let dir = TempDir::new("scan_ignore");
        dir.write(".gitignore", "drafts/\n");
        dir.write("keep.cdrm", "# Keep");
        dir.write("drafts/skip.cdrm", "# Skip");
        dir.write(".hidden/secret.cdrm", "# Hidden");
        let root = SyncRoot::open(dir.path().to_str().unwrap()).unwrap();

        assert_eq!(paths(&scan(&root, &options())), vec!["keep.cdrm"]);

        let everything = ScanOptions { respect_gitignore: false,
                                       include_hidden: true,
                                       file_kinds: NoteFileKind::all() };
        assert_eq!(paths(&scan(&root, &everything)), vec![".hidden/secret.cdrm", "drafts/skip.cdrm", "keep.cdrm"]);
    }

    #[test]
    fn filters_by_requested_kind() {
        let dir = TempDir::new("scan_kinds");
        dir.write("a.cdrm", "# A");
        dir.write("b.md", "# B");
        let root = SyncRoot::open(dir.path().to_str().unwrap()).unwrap();
        let only_cdrm = ScanOptions { file_kinds: vec![NoteFileKind::Cdrm],
                                      ..options() };
        assert_eq!(paths(&scan(&root, &only_cdrm)), vec!["a.cdrm"]);
    }

    #[test]
    fn reports_unusable_files_as_skipped() {
        let dir = TempDir::new("scan_skipped");
        std::fs::write(dir.path().join("bad.md"), [0xff, 0xfe]).unwrap();
        let root = SyncRoot::open(dir.path().to_str().unwrap()).unwrap();
        let response = scan(&root, &options());
        assert!(response.entries.is_empty());
        assert_eq!(response.skipped.len(), 1);
        assert_eq!(response.skipped[0].relative_path, "bad.md");
    }
}
