use std::fmt::Display;

use katex::{KatexContext, Settings, TrustSetting, render_to_string};
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
            },
            traits::{
                conundrum_input::{ArcState, ConundrumInput},
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

    pub fn to_children(&self, state: ArcState) -> ConundrumModalResult<Children> {
        let mut nested_state = ConundrumInput { input: self.0.as_str(),
                                                state };
        let res = parse_elements(&mut nested_state)?;
        Ok(Children(res))
    }

    pub fn to_math(&self, block_level: bool, trusted: bool) -> ConundrumModalResult<String> {
        let context = KatexContext::default();
        let settings = Settings::builder().display_mode(block_level)
                                          .trust(TrustSetting::Bool(trusted))
                                          .color_is_text_color(true)
                                          .build();

        render_to_string(&context, &self.0, &settings).map_err(|e| {
            eprintln!("Error: {:#?}", e);
            ErrMode::Cut(
                ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Math Error", format!(r#"Conundrum could not compile a math block with the following content:

```tex
{}
```
                        "#, self.0).as_str()))
            )
        })
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
    /// Sets the property as a Fragment here. Use the `to_jsx_prop_as_string`
    /// method to set the property as a string.
    fn to_jsx_prop(&self, key: &str) -> String {
        format!("{}={{<>{}</>}}", key, self.0)
    }
}

impl PlainTextComponentResult for ConundrumString {
    fn to_plain_text(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}

impl JsxComponentResult for ConundrumString {
    fn to_jsx_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("\"{}\"", self.to_quoted_string().unwrap_or(String::from(""))))
    }
}

impl ConundrumComponentResult for ConundrumString {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.is_markdown_or_plain_text() {
            drop(state);
            self.to_plain_text(res)
        } else {
            drop(state);
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
