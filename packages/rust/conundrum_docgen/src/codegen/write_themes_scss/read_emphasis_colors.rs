use std::collections::HashMap;

use crate::workspace_utils::get_workspace_root_duplicate::get_workspace_root;

#[derive(serde::Deserialize)]
pub struct EmphasisEntryColorGroup {
    pub background: String,
    pub foreground: String,
}

#[derive(serde::Deserialize)]
pub struct EmphasisEntry {
    pub light: EmphasisEntryColorGroup,
    pub dark: EmphasisEntryColorGroup,
}

pub type EmphasisColorMap = HashMap<String, EmphasisEntry>;

pub fn get_emphasis_color_entries() -> EmphasisColorMap {
    let root = get_workspace_root();
    let p = std::path::Path::new(&root).join("packages")
                                       .join("webview_utils")
                                       .join("src")
                                       .join("core")
                                       .join("styles")
                                       .join("emphasis_colors.json");
    let content = std::fs::read_to_string(p).expect("Reads emphasis_colors.json without throwing an error.");
    let y: HashMap<String, EmphasisEntry> =
        serde_json::from_str(&content).expect("Deserializes emphasis_colors.json without throwing an error.");
    y
}
