use crate::traits::DocGenTemplate;
use askama::Template;
use conundrum::lang::runtime::traits::conundrum_template::CDRMTemplateRepresentableWithParam;
use strum::IntoEnumIterator;

/// ## Template (Conundrum)
#[derive(Template)]
#[template(ext = "jinja", path = "markdown/documentation/underline.txt")]
pub struct UnderlineDocs {}

impl DocGenTemplate for UnderlineDocs {
    fn gather_data() -> Self {
        Self {}
    }

    fn descriptive_label() -> String {
        String::from("underline docs")
    }
}
