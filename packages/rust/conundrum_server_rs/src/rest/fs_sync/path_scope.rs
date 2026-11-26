//! The trust boundary for every path a client sends us.
use std::path::{Component, Path, PathBuf};

use crate::rest::fs_sync::{error::FsSyncError, models::FileIssueReason};

/// A canonicalized directory that all file access for a request is confined
/// to. Construct one per request; never join client input onto a path any
/// other way.
#[derive(Debug, Clone)]
pub struct SyncRoot {
    canonical: PathBuf,
}

impl SyncRoot {
    pub fn open(raw: &str) -> Result<SyncRoot, FsSyncError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(FsSyncError::InvalidRoot(raw.to_string()));
        }
        let canonical = std::fs::canonicalize(trimmed).map_err(|_| FsSyncError::InvalidRoot(raw.to_string()))?;
        if !canonical.is_dir() {
            return Err(FsSyncError::InvalidRoot(raw.to_string()));
        }
        Ok(SyncRoot { canonical })
    }

    pub fn path(&self) -> &Path {
        &self.canonical
    }

    pub fn display(&self) -> String {
        self.canonical.to_string_lossy().to_string()
    }

    /// Resolves a client-supplied relative path to an existing file inside
    /// the root. Rejects absolute paths, `..` segments, and symlinks that
    /// escape the root.
    pub fn resolve(&self, relative: &str) -> Result<PathBuf, FileIssueReason> {
        let relative_path = Path::new(relative);
        let is_plain_relative = !relative.is_empty()
                                && relative_path.components()
                                                .all(|c| matches!(c, Component::Normal(_) | Component::CurDir));
        if !is_plain_relative {
            return Err(FileIssueReason::OutsideRoot);
        }
        let resolved = std::fs::canonicalize(self.canonical.join(relative_path)).map_err(|e| {
                           match e.kind() {
                               std::io::ErrorKind::NotFound => FileIssueReason::NotFound,
                               _ => FileIssueReason::Unreadable,
                           }
                       })?;
        if !resolved.starts_with(&self.canonical) {
            return Err(FileIssueReason::OutsideRoot);
        }
        Ok(resolved)
    }

    /// The `/`-separated path of `absolute` relative to the root, or `None`
    /// if it is outside the root or not valid UTF-8.
    pub fn relativize(&self, absolute: &Path) -> Option<String> {
        let stripped = absolute.strip_prefix(&self.canonical).ok()?;
        let mut parts: Vec<&str> = Vec::new();
        for component in stripped.components() {
            match component {
                Component::Normal(part) => parts.push(part.to_str()?),
                Component::CurDir => {}
                _ => return None,
            }
        }
        if parts.is_empty() {
            return None;
        }
        Some(parts.join("/"))
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use std::path::{Path, PathBuf};

    /// A uniquely named directory under the system temp dir, removed on drop.
    pub struct TempDir(pub PathBuf);

    impl TempDir {
        pub fn new(label: &str) -> TempDir {
            let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                                                    .map(|d| d.as_nanos())
                                                    .unwrap_or(0);
            let path = std::env::temp_dir().join(format!("cdrm_fs_sync_{}_{}_{}", label, std::process::id(), nanos));
            std::fs::create_dir_all(&path).expect("create temp dir");
            TempDir(path)
        }

        pub fn write(&self, relative: &str, content: &str) -> PathBuf {
            let path = self.0.join(relative);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).expect("create parent dir");
            }
            std::fs::write(&path, content).expect("write file");
            path
        }

        pub fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{test_support::TempDir, *};

    #[test]
    fn rejects_missing_or_empty_roots() {
        assert!(SyncRoot::open("").is_err());
        assert!(SyncRoot::open("/definitely/not/a/real/dir/for/cdrm").is_err());
    }

    #[test]
    fn rejects_a_file_as_root() {
        let dir = TempDir::new("file_root");
        let file = dir.write("note.cdrm", "# Hi");
        assert!(SyncRoot::open(file.to_str().unwrap()).is_err());
    }

    #[test]
    fn resolves_paths_inside_the_root() {
        let dir = TempDir::new("resolve_inside");
        dir.write("nested/note.cdrm", "# Hi");
        let root = SyncRoot::open(dir.path().to_str().unwrap()).unwrap();
        let resolved = root.resolve("nested/note.cdrm").unwrap();
        assert_eq!(root.relativize(&resolved).as_deref(), Some("nested/note.cdrm"));
    }

    #[test]
    fn rejects_escaping_paths() {
        let dir = TempDir::new("resolve_escape");
        dir.write("note.cdrm", "# Hi");
        let root = SyncRoot::open(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(root.resolve("../note.cdrm").unwrap_err(), FileIssueReason::OutsideRoot);
        assert_eq!(root.resolve("nested/../../note.cdrm").unwrap_err(), FileIssueReason::OutsideRoot);
        assert_eq!(root.resolve("/etc/hosts").unwrap_err(), FileIssueReason::OutsideRoot);
        assert_eq!(root.resolve("").unwrap_err(), FileIssueReason::OutsideRoot);
        assert_eq!(root.resolve("missing.cdrm").unwrap_err(), FileIssueReason::NotFound);
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlinks_that_escape_the_root() {
        let outside = TempDir::new("symlink_target");
        outside.write("secret.cdrm", "secret");
        let dir = TempDir::new("symlink_root");
        std::os::unix::fs::symlink(outside.path().join("secret.cdrm"), dir.path().join("link.cdrm")).unwrap();
        let root = SyncRoot::open(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(root.resolve("link.cdrm").unwrap_err(), FileIssueReason::OutsideRoot);
    }
}
