use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use winnow::{
    ModalResult,
    error::{ErrMode, ParserError},
};

use crate::{
    ecosystem::environment_variables::cdrm_env_variable::CdrmEnvVariable,
    lang::runtime::{state::conundrum_error::ConundrumError, traits::conundrum_input::ConundrumInput},
    output::general::component_constants::any_component_id::AnyComponentName,
};

#[typeshare::typeshare]
#[derive(Debug, Error, uniffi::Error, Serialize, Deserialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum ConundrumErrorVariant {
    #[error("Conundrum runs on a background thread by default, and it ran into some issues in your environment.")]
    MultiThreadingError,
    #[error("Conundrum could not convert the `{from}` type to a `{to}` type.")]
    TypeConversionError {
        from: String,
        to: String,
    },
    #[error("Codeblock meta data does not exist.")]
    CodeblockMetaDataNotExist,
    #[error("The specified key could not be found in the dictionary it was being used against.")]

    KeyNotFound,
    #[error("This should never happen, some embedded data can not be found.")]
    EmbeddedDataNotFound,
    #[error("Conundrum could not find a valid terminator.")]
    TerminatorNotFound,
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
    /// Deprecated in favor of MissingComponentProperty and
    /// InvalidComponentProperty.
    #[error("A provided property seems to be missing or incorrectly applied.")]
    UserFacingMissingOrIncorrectProperty(ConundrumError),
    #[error("The `{component}` component is expecting an `{expected}` type at the `{key}` key. Check the `{component}??` documentation for more information.")]
    MissingRequiredComponentProperty {
        component: AnyComponentName,
        key: String,
        expected: String,
    },
    #[error("The `{component}` component received an invalid property at the `{key}` key. Conundrum expected a `{expected}` but received `{received}`")]
    InvalidComponentProperty {
        component: AnyComponentName,
        key: String,
        expected: String,
        /// If the type is not inferible, use `an unknown type` to align with
        /// the error verbiage.
        received: String,
    },

    #[error("The value of `{0}` isn't a valid css variable. Valid variables must start with two leading `--` characters.")]
    InvalidCSSVariableSyntax(String),
    #[error("This is a general parser fail. We can do much better with these error messages.")]
    InternalParserError(ConundrumError),
    #[error("Environment variable not found: `{0}`")]
    EnvVarNotFound(CdrmEnvVariable),
    #[error("Emoji render error: {0}")]
    EmojiRenderError(String),
    #[error("WASM runtime error: {0}")]
    WasmError(String),
    #[error("The provided string of `{0}` could not be parsed successfully to a CSS supported color.")]
    InvalidColor(String),
    #[error("There seems to be an error in your mermaid syntax. This is the error from mermaid: {0}")]
    MermaidError(String),
}

impl From<ErrMode<ConundrumErrorVariant>> for ConundrumErrorVariant {
    fn from(value: ErrMode<ConundrumErrorVariant>) -> Self {
        match value {
            ErrMode::Backtrack(b) => b,
            ErrMode::Cut(c) => c,
            ErrMode::Incomplete(_) => {
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
