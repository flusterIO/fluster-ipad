use std::path::PathBuf;

use crate::workspace_utils::get_workspace_root_duplicate::get_workspace_root_as_pathbuf_docgen;

pub struct RepoManager {
    pub root: PathBuf,
}

impl RepoManager {
    pub fn new() -> Self {
        Self { root: get_workspace_root_as_pathbuf_docgen() }
    }

    pub fn workspace_relative_path(&self, relative_path: String) -> PathBuf {
        self.root.join(relative_path)
    }

    pub fn get_themes_css_content(&self) -> String {
        let p = self.workspace_relative_path("packages/webview_utils/src/core/styles/themes.scss".to_string());

        std::fs::read_to_string(p).expect("Failed to read themes scss file to a string.")
    }
}
