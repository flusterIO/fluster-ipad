use serde::Serialize;
use strum::Display;
use typeshare::typeshare;

/// To enforce some uniformity and ease-of-use between components, this enum
/// represents a list of commonly used property keys. The code itself doesn't
/// care that these keys are the same, but the predictability should help new
/// users.
#[typeshare]
#[derive(Debug, Serialize, Display)]
pub enum CommonComponentPropertyKey {
    #[serde(rename = "markdownHeading")]
    #[strum(to_string = "markdownHeading")]
    MarkdownHeading,
    #[serde(rename = "markdown")]
    #[strum(to_string = "markdown")]
    InlineMarkdownOverride,
}
