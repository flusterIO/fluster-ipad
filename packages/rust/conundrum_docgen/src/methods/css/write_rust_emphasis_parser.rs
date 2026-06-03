use askama::Template;
use strum::IntoEnumIterator;

use crate::{
    errors::{DocGenError, DocGenResult},
    traits::DocGenTemplate,
    workspace_utils::get_workspace_root_duplicate::get_workspace_root,
};

/// This works, but the Emphasis json file is missing the Card and Primary
/// variants. If the emphasis list grows, add the variants here and do it
/// through generation. That was a big mistake not doing themes in json from the
/// beginning.
#[derive(Template, serde::Deserialize)]
#[template(path = "rust/emphasis_parser.txt", ext = "jinja")]
pub struct RustEmphasisParserTemplate {}

impl DocGenTemplate for RustEmphasisParserTemplate {
    fn gather_data() -> Self {
        RustEmphasisParserTemplate {}
    }

    fn descriptive_label() -> String {
        String::from("emphasis parser")
    }
}
