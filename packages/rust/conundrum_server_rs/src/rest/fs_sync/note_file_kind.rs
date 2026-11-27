use std::path::Path;

use conundrum_lang::lang::constants::file_types::ParsableFileType;
use serde::{Deserialize, Serialize};

/// The subset of [`ParsableFileType`] that maps onto a text note.
///
/// To sync a new kind of file, add a variant here and to the `match` in
/// [`NoteFileKind::from_parsable`]. Clients that don't request the new kind
/// are unaffected.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NoteFileKind {
    #[serde(rename = "cdrm")]
    Cdrm,
    #[serde(rename = "md")]
    Markdown,
    #[serde(rename = "mdx")]
    Mdx,
}

impl NoteFileKind {
    pub fn all() -> Vec<NoteFileKind> {
        vec![NoteFileKind::Cdrm, NoteFileKind::Markdown, NoteFileKind::Mdx]
    }

    pub fn from_parsable(file_type: ParsableFileType) -> Option<NoteFileKind> {
        match file_type {
            ParsableFileType::Cdrm => Some(NoteFileKind::Cdrm),
            ParsableFileType::Markdown => Some(NoteFileKind::Markdown),
            ParsableFileType::Mdx => Some(NoteFileKind::Mdx),
            _ => None,
        }
    }

    /// Resolves the kind from a path's extension, reusing Conundrum's own
    /// extension rules so the two never drift apart.
    pub fn from_path(path: &Path) -> Option<NoteFileKind> {
        let extension = path.extension()?.to_str()?;
        let parsable = ParsableFileType::try_from(extension.to_string()).ok()?;
        NoteFileKind::from_parsable(parsable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_note_kinds_from_extensions() {
        assert_eq!(NoteFileKind::from_path(Path::new("a/b.cdrm")), Some(NoteFileKind::Cdrm));
        assert_eq!(NoteFileKind::from_path(Path::new("a/b.MD")), Some(NoteFileKind::Markdown));
        assert_eq!(NoteFileKind::from_path(Path::new("b.mdx")), Some(NoteFileKind::Mdx));
        assert_eq!(NoteFileKind::from_path(Path::new("b.pdf")), None);
        assert_eq!(NoteFileKind::from_path(Path::new("b.txt")), None);
        assert_eq!(NoteFileKind::from_path(Path::new("no_extension")), None);
    }
}
