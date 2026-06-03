use askama::Template;

use crate::{
    codegen::write_themes_scss::read_emphasis_colors::{EmphasisColorMap, get_emphasis_color_entries},
    traits::DocGenTemplate,
};

/// This works, but the Emphasis json file is missing the Card and Primary
/// variants. If the emphasis list grows, add the variants here and do it
/// through generation. That was a big mistake not doing themes in json from the
/// beginning.
#[derive(Template, serde::Deserialize)]
#[template(path = "rust/emphasis_to_default_color_group.txt", ext = "jinja")]
pub struct RustEmphasisToColorGroupTemplate {
    pub items: EmphasisColorMap,
}

impl DocGenTemplate for RustEmphasisToColorGroupTemplate {
    fn gather_data() -> Self {
        RustEmphasisToColorGroupTemplate { items: get_emphasis_color_entries() }
    }

    fn descriptive_label() -> String {
        String::from("Emphasis to color group")
    }
}
