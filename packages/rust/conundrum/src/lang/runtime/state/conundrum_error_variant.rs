use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use winnow::{
    ModalResult,
    error::{ErrMode, ParserError},
};

use crate::lang::runtime::{state::conundrum_error::ConundrumError, traits::conundrum_input::ConundrumInput};

#[typeshare::typeshare]
#[derive(Debug, Error, uniffi::Error, Serialize, Deserialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum ConundrumErrorVariant {
    #[error("Conundrum runs on a background thread by default, and it ran into some issues in your environment.")]
    MultiThreadingError,
    #[error("It doesn't' appear as if this note has a valid title.")]
    FailToFindTitleGroup,
    #[error("Failed to find the component {0}")]
    FailToFindComponent(String),
    #[error("Conundrum attempted to create a cross-language string safefly and could not do so.")]
    FailToGenerateString,
    #[error("There was an error parsing your front-matter. Please see the `FrontMatter??` docs for help.")]
    FrontMatterError,
    #[error("There was an error parsing this content")]
    UserFacingGeneralParserError(ConundrumError),
    #[error("A provided property seems to be missing or incorrectly applied.")]
    UserFacingMissingOrIncorrectProperty(ConundrumError),
    #[error("This is a general parser fail. We can do much better with these error messages.")]
    InternalParserError(ConundrumError),
}

impl From<ErrMode<ConundrumErrorVariant>> for ConundrumErrorVariant {
    fn from(value: ErrMode<ConundrumErrorVariant>) -> Self {
        match value {
            ErrMode::Backtrack(b) => b,
            ErrMode::Cut(c) => c,
            ErrMode::Incomplete(i) => {
                ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("General parsing error.",
                                                                                                "To be honest I'm not sure when this error will be reached. I'm building this all without the internet so I can't look things up..."))
            }
        }
    }
}

pub type ConundrumResult<T> = Result<T, ConundrumErrorVariant>;
pub type ConundrumModalResult<T> = ModalResult<T, ConundrumErrorVariant>;

// impl ConundrumErrorVariant {
//     // Avoiding `From` so `winnow` types don't become part of our public API
//     fn from_parse(error: ParseError<&str, ContextError>) -> Self {
//         // The default renderer for `ContextError` is still used but that can
// be         // customized as well to better fit your needs.
//         let message = error.inner().to_string();
//         let input = (*error.input()).to_owned();
//         println!("Error Input?: {}", input);
//         // Assume the error span is only for the first `char`.
//         let span = error.char_span();
//         let err = ConundrumError { msg: "Conundrum failure".to_string(),
//                                    pos: Some(DocumentSpan::from(span)),
//                                    details: Some(message) };

//         ConundrumErrorVariant::InternalParserError(err)
//     }
// }
// impl From<ErrMode<ContextError>> for ConundrumErrorVariant {
//     fn from(value: ErrMode<ContextError>) -> Self {
//         ConundrumErrorVariant::InternalParserError(ConundrumError::from(value))
//     }
// }

// impl From<ParseError<&str, ContextError>> for ConundrumErrorVariant {
//     fn from(value: ParseError<&str, ContextError>) -> Self {
//         let x = ConundrumError::from(value);
//         ConundrumErrorVariant::InternalParserError(x)
//     }
// }
//

impl ParserError<&str> for ConundrumErrorVariant {
    type Inner = ConundrumErrorVariant;

    fn from_input(_: &&str) -> Self {
        ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("This is a general Conundrum parser error that should never make it back to the user."))
    }

    fn into_inner(self) -> winnow::Result<Self::Inner, Self> {
        Ok(self)
    }
}

impl ParserError<ConundrumInput<'_>> for ConundrumErrorVariant {
    type Inner = ConundrumErrorVariant;

    fn from_input(_: &ConundrumInput) -> Self {
        ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("This is a general Conundrum parser error that should never make it back to the user."))
    }

    fn into_inner(self) -> winnow::Result<Self::Inner, Self> {
        Ok(self)
    }
}
