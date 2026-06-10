use crate::{traits::DocGenTemplate};
use askama::Template;
use conundrum::lang::runtime::traits::conundrum_template::CDRMTemplateRepresentableWithParam;
use strum::IntoEnumIterator;

/// ## Template (Conundrum)
#[derive(Template)]
#[template(ext = "jinja", path = "markdown/documentation/highlight.txt")]
pub struct HighlightDocs {}

impl DocGenTemplate for HighlightDocs {
    fn gather_data() -> Self {
        Self {}
    }

    fn descriptive_label() -> String {
        String::from("highlight docs")
    }
}
