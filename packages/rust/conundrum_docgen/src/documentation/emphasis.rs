use crate::{traits::DocGenTemplate, workspace_utils::get_workspace_root_duplicate::get_workspace_root};
use askama::Template;
use conundrum::lang::runtime::traits::conundrum_template::CDRMTemplateRepresentableWithParam;
use strum::IntoEnumIterator;

/// ## Template (Conundrum)
#[derive(Template)]
#[template(ext = "jinja", path = "markdown/documentation/emphasis.txt")]
pub struct EmphasisDocs {}

impl DocGenTemplate for EmphasisDocs {
    fn gather_data() -> Self {
        Self {}
    }

    fn descriptive_label() -> String {
        String::from("emphasis docs")
    }
}
