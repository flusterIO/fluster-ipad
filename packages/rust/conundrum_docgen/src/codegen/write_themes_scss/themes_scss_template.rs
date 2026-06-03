use std::collections::HashMap;

use askama::Template;

use crate::codegen::write_themes_scss::read_emphasis_colors::EmphasisEntry;

#[derive(Template, serde::Deserialize)]
#[template(path = "css/themes.txt", ext = "jinja")]
pub struct ThemesScssTemplate {
    pub items: Vec<EmphasisEntry>,
}
