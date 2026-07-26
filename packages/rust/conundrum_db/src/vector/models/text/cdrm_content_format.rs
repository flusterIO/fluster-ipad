use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, strum_macros::Display)]
pub enum CdrmContentFormat {
    #[strum(to_string = "markdown")]
    #[serde(rename = "markdown")]
    Markdown,
    /// Content that was parsed to Markdown from html.
    /// Future plans include a parser package that will accept functions,
    /// basically css queries and map them to components.
    #[strum(to_string = "cdrm_from_html")]
    #[serde(rename = "cdrm_from_html")]
    CdrmFromHtml,
    #[strum(to_string = "cdrm")]
    #[serde(rename = "cdrm")]
    Conundrum,
}
