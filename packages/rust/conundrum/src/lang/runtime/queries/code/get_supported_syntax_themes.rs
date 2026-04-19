use askama::Template;
use syntect::highlighting::ThemeSet;

use crate::lang::runtime::state::parse_state::ParseState;

pub fn get_supported_syntax_themes() -> Vec<String> {
    let s = ParseState::default();
    s.highlight_assets.themes().map(String::from).collect::<Vec<String>>()
}

/// ```askama
/// use serde::{Deserialize, Serialize};
/// use strum::EnumIter;
/// use strum_macros::{Display, EnumString};
/// use syntect::highlighting::{Theme, ThemeSet};

/// #[typeshare::typeshare]
/// #[derive(Serialize, Deserialize, Display, EnumString, EnumIter, uniffi::Enum, Clone, Default, Debug)]
/// #[allow(non_camel_case_types)]
/// pub enum SupportedCodeBlockTheme { {% for name in names %}
/// #[serde(rename = "{{name}}")]
/// #[strum(to_string = "{{name}}")]
/// {{name | capitalize}},{% endfor %}
/// ```
#[derive(Template)]
#[template(in_doc = true, ext = "html")]
pub struct T {
    pub names: Vec<String>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn gets_supported_syntax_themes() {
        let res = get_supported_syntax_themes();
        let r = T { names: res.clone() };
        let s = r.render().expect("Fuck me");
        println!("{}", s);
        // assert!(!res.is_empty(), "Returns valid syntax themes.");
    }
}
