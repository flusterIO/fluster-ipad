use askama::Template;
// use css_color::

use crate::traits::DocGenTemplate;

#[derive(Template)]
#[template(ext = "jinja", path = "rust/parsers/color.txt")]
pub struct ColorParserTemplate {}

impl DocGenTemplate for ColorParserTemplate {
    fn gather_data() -> Self {
        Self {}
    }

    fn descriptive_label() -> String {
        String::from("color parser")
    }
}
