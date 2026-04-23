#[derive(Clone)]
pub struct SupportedThemeItem {
    pub lang: String,
    pub extensions: Vec<String>,
}

pub fn get_supported_syntaxes() -> Vec<SupportedThemeItem> {
    let hl = syntect_assets::assets::HighlightingAssets::from_binary();
    hl.get_syntaxes()
      .expect("Returns embedded syntaxes without throwing an error.")
      .iter()
      .map(|item| SupportedThemeItem { lang: item.name.clone(),
                                       extensions: item.file_extensions.clone() })
      .collect::<Vec<SupportedThemeItem>>()
}

#[cfg(test)]
mod tests {

    use std::{fmt::Display, ops::Index};

    use askama::Template;
    use tabled::{Table, Tabled, settings::Style, settings::Theme};

    use crate::testing::get_workspace_root::get_workspace_root;

    use super::*;

    #[test]
    fn gets_supported_syntaxes() {
        let res = get_supported_syntaxes();
    }
}
