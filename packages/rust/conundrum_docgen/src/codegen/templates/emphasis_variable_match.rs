use askama::Template;
use conundrum::{
    lang::lib::ui::ui_types::emphasis::emphasis_model::Emphasis,
    output::html::web_specific_traits::css_value_representable::CSSVariablePairRepresentable,
};
use strum::IntoEnumIterator;

use crate::traits::DocGenTemplate;

#[derive(Template)]
#[template(path = "rust/emphasis_variable_match.txt", ext = "jinja")]
pub struct EmphasisVariableMatch {}

impl DocGenTemplate for EmphasisVariableMatch {
    fn descriptive_label() -> String {
        String::from("Emphasis variable match")
    }

    fn gather_data() -> Self {
        Self {}
    }
}
