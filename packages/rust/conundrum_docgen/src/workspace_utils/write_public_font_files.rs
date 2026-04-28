use std::path::Path;

use conundrum::testing::get_workspace_root_duplicate::get_workspace_root;

pub fn write_public_font_file<C: AsRef<[u8]>>(content: C, sub_path: &str) {
    let paths = vec!["packages/webviews/bib_entry_details_webview/public",
                     "packages/webviews/bibtex_editor_webview/public",
                     "packages/webviews/dictionary_webview/public",
                     "packages/webviews/note_detail_webview/public",
                     "packages/webviews/splitview_mdx_editor/public",
                     "packages/webviews/standalone_mdx_editor/public",
                     "packages/webviews/standalone_mdx_preview/public",];
    for p in paths {
        let absolute_path = Path::new(&get_workspace_root()).join(p).join(sub_path);
        std::fs::write(absolute_path, &content).expect(
                                                       "Expected to write font
        file without throwing an error.",
        );
    }
}

#[cfg(test)]
mod tests {
    use conundrum::output::html::glue::individual_glue_assets::lucide_font::{self, LucideFontAsset};

    use super::*;

    #[test]
    fn write_font_files() {
        write_public_font_file(LucideFontAsset::as_bytes(), "lucide.ttf");
        let root = get_workspace_root();
        let devicon_font_path = std::path::Path::new(&root).join("packages")
                                                           .join("rust")
                                                           .join("conundrum")
                                                           .join("assets")
                                                           .join("fonts")
                                                           .join("Fira Code Regular Nerd Font Complete.ttf");
        let di = std::fs::read(devicon_font_path).expect("Reads devicons font.");
        write_public_font_file(di, "nerd.ttf");
        // assert_eq!(result, 4);
    }
}
