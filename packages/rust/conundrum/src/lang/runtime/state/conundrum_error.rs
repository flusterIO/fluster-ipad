use std::{fmt::Display, ops::Range};

use serde::{Deserialize, Serialize};
use winnow::error::{ContextError, ErrMode, ParseError};

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, Debug, uniffi::Record, Clone)]
pub struct DocumentSpan {
    pub start: u32,
    pub end: u32,
}

impl From<Range<usize>> for DocumentSpan {
    fn from(value: Range<usize>) -> Self {
        let start = value.start as u32;
        let end = value.end as u32;
        DocumentSpan { start,
                       end }
    }
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, Debug, uniffi::Enum, Clone, strum_macros::Display)]
pub enum ConundrumErrorPurpose {
    #[serde(rename = "info")]
    #[strum(to_string = "info")]
    Info,
    #[serde(rename = "warn")]
    #[strum(to_string = "warn")]
    Warn,
    #[serde(rename = "suggestion")]
    #[strum(to_string = "suggestion")]
    Suggest,
    #[serde(rename = "syntax")]
    #[strum(to_string = "syntax")]
    SyntaxError,
    #[serde(rename = "config")]
    #[strum(to_string = "config")]
    ConfigurationBug,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, Debug, uniffi::Record, Clone)]
pub struct ConundrumError {
    /// A required message displayed in a toast or card header. Can contain
    /// regular, swift-inline compatible markdown only.
    pub msg: String,
    pub pos: Option<DocumentSpan>,
    /// Details is an optional conundrum string offering a more thorough
    /// explanation to the user if one can be generated.
    pub details: Option<String>,

    /// An indicator that can be used when the warning UI is implemented to show
    /// user's errors that allowed their notes to still compile. There's
    /// lot's of useful information that can be gathered from these errors,
    /// inclduding property suggestions & more.
    ///
    /// Making it optional for now because I'm too lazy to go back and rewrite
    /// all of the errors.
    pub purpose: Option<ConundrumErrorPurpose>,
}

impl Display for ConundrumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl ConundrumError {
    // Avoiding `From` so `winnow` types don't become part of our public API
    fn from_parse(error: ParseError<&str, ContextError>) -> Self {
        // The default renderer for `ContextError` is still used but that can be
        // customized as well to better fit your needs.
        let message = error.inner().to_string();
        let input = (*error.input()).to_owned();
        // Assume the error span is only for the first `char`.
        let span = error.char_span();
        Self { msg: message,
               pos: Some(DocumentSpan::from(span)),
               details: None,
               purpose: None }
    }

    pub fn general_render_error() -> Self {
        ConundrumError::from_msg_and_details("Invalid Heading",
                                             "An unknown rendering error occurred. Conundrum ams to have great, helpful error reporting but this is not one of those cases unfortunately. If this issue continues, please file an issue on Github.")
    }
}

impl std::error::Error for ConundrumError {}

impl From<ErrMode<ContextError>> for ConundrumError {
    fn from(value: ErrMode<ContextError>) -> Self {
        ConundrumError { msg: "This is a general Conundrum
Error"
                              .to_string(),
                         pos: None,
                         purpose: None,
                         details: Some(value.to_string()) }
    }
}

impl From<ParseError<&str, ContextError>> for ConundrumError {
    fn from(value: ParseError<&str, ContextError>) -> Self {
        ConundrumError { msg: "This is a general Conundrum
Error"
                              .to_string(),
                         pos: None,
                         purpose: None,
                         details: Some(value.to_string()) }
    }
}

impl ConundrumError {
    pub fn from_msg_and_details(msg: &str, details: &str) -> Self {
        ConundrumError { msg: msg.to_string(),
                         pos: None,
                         purpose: None,
                         details: Some(details.to_string()) }
    }

    pub fn from_message(msg: &str) -> Self {
        ConundrumError { msg: msg.to_string(),
                         pos: None,
                         purpose: None,
                         details: None }
    }
}
