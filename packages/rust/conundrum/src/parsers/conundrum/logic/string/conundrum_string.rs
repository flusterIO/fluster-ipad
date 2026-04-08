use std::fmt::Display;

use serde::{Deserialize, Serialize};
use winnow::{
    Parser,
    combinator::{alt, delimited},
    error::ErrMode,
    token::take_till,
};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            ui_traits::jsx_prop_representable::{FromJsxPropsOptional, JsxPropRepresentable},
            ui_types::children::Children,
        },
        runtime::{
            parse_conundrum_string::parse_elements,
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult, ConundrumResult},
                parse_state::{ConundrumModifier, ParseState},
            },
            traits::{
                conundrum_input::{ConundrumInput, get_conundrum_input},
                fluster_component_result::ConundrumComponentResult,
                jsx_component_result::JsxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    parsers::{
        conundrum::logic::{object::object::ConundrumObject, token::ConundrumLogicToken},
        javascript::javascript_parser_trait::JavascriptParser,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ConundrumString(pub String);

uniffi::custom_newtype!(ConundrumString, String);

impl Display for ConundrumString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<&str> for ConundrumString {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl ConundrumString {
    pub fn new(content: &str) -> Self {
        ConundrumString(content.to_string())
    }

    pub fn to_children(&self, modifiers: Vec<ConundrumModifier>) -> ConundrumModalResult<Children> {
        let mut state = get_conundrum_input(&self.0, modifiers);
        let res = parse_elements(&mut state)?;
        Ok(Children(res))
    }

    /// Returns a string that is ***already*** wrapped in quotes, with all
    /// characters escaped.
    pub fn to_quoted_string(&self) -> ConundrumResult<String> {
        serde_json::to_string(&self.0).map_err(|e| {
                                          println!("Error: {:#?}", e);
                                          ConundrumErrorVariant::FailToGenerateString
                                      })
    }

    pub fn double_quoted_javascript_string(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumString> {
        let res = delimited('"', take_till(0.., |c| c == '\n' || c == '"'), '"').parse_next(input)?;
        Ok(ConundrumString(res.to_string()))
    }

    pub fn single_quoted_javascript_string(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumString> {
        let res = delimited('\'', take_till(0.., |c| c == '\n' || c == '\''), '\'').parse_next(input)?;
        Ok(ConundrumString(res.to_string()))
    }

    pub fn back_tick_quoted_javascript_string(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumString> {
        let res = delimited('`', take_till(0.., |c| c == '`'), '`').parse_next(input)?;
        Ok(ConundrumString(res.to_string()))
    }

    pub fn to_jsx_prop_as_string(&self, key: &str) -> ConundrumResult<String> {
        let s = self.to_quoted_string()?;
        Ok(format!("{}={}", key, s))
    }
}

impl JsxPropRepresentable for ConundrumString {
    fn to_jsx_prop(&self, key: &str) -> String {
        format!("{}={{<>{}</>}}", key, self.0)
    }
}

impl PlainTextComponentResult for ConundrumString {
    fn to_plain_text(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}

impl JsxComponentResult for ConundrumString {
    fn to_jsx_component(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(format!("\"{}\"", self.to_quoted_string().unwrap_or(String::from(""))))
    }
}

impl ConundrumComponentResult for ConundrumString {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if res.is_markdown_or_plain_text() {
            self.to_plain_text(res)
        } else {
            self.to_jsx_component(res)
        }
    }
}

impl JavascriptParser<ConundrumString> for ConundrumString {
    fn parse_javascript(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumString> {
        let s = alt((ConundrumString::back_tick_quoted_javascript_string,
                     ConundrumString::single_quoted_javascript_string,
                     ConundrumString::double_quoted_javascript_string)).parse_next(input)?;
        Ok(s)
    }
}

impl FromJsxPropsOptional for ConundrumString {
    fn from_jsx_props(props: &ConundrumObject, key: &str) -> ConundrumModalResult<Self>
        where Self: Sized {
        if let Some(res) = props.data.get(key) {
            let res = match res.value() {
                ParsedElement::Logic(s) => match s {
                    ConundrumLogicToken::String(s) => Some(s),
                    _ => None,
                },
                _ => None,
            };
            match res {
                Some(r) => Ok(r.clone()),
                None => {
                    Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Expected a `String`, found something else.", format!("Conundrum was looking for a string at the `{}` key but found something else. Please review the `Syntax??` docs if you're stuck.", key).as_str()))))
                }
            }
        } else {
            Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Expected a `String`, found something nothing.", format!("Conundrum was looking for a string at the `{}` key but couldn't find anything. Please review the `Syntax??` docs or the documentation for this specific component if you're stuck.", key).as_str()))))
        }
    }
}
