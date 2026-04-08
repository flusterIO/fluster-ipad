use std::str::FromStr;

use dashmap::DashMap;
use serde::Serialize;
use winnow::{
    Parser,
    ascii::space0,
    combinator::{delimited, repeat},
};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::shared_props::sizable_option::SizableOption,
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult, ConundrumResult},
            },
            traits::conundrum_input::ConundrumInput,
        },
    },
    parsers::{
        conundrum::logic::{
            bool::boolean::ConundrumBoolean, number::conundrum_number::ConundrumNumber,
            string::conundrum_string::ConundrumString, token::ConundrumLogicToken,
        },
        javascript::object::javascript_key_value_pair::JavascriptObjectKeyValuePair,
        react::parser_components::jsx_properties::any_jsx_property::any_jsx_property,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct ConundrumObject {
    pub data: DashMap<String, ParsedElement>,
}

impl ConundrumObject {
    pub fn from_kv_pair_vec(entries: Vec<JavascriptObjectKeyValuePair>) -> Self {
        let data = DashMap::new();
        for entry in entries {
            data.insert(entry.key, *entry.value);
        }
        ConundrumObject { data }
    }

    /// This only parses property strings that **do not** accept line breaks.
    pub fn from_single_line_property_string_parser(input: &mut ConundrumInput)
                                                   -> ConundrumModalResult<ConundrumObject> {
        let entries: Vec<JavascriptObjectKeyValuePair> =
            repeat(0.., delimited(space0, any_jsx_property, space0)).parse_next(input)?;
        let s = ConundrumObject::from_kv_pair_vec(entries);
        Ok(s)
    }

    /// Same as `get_string`, but accepts an error callback instead of an error
    /// message.
    pub fn get_string_with_err_cb(&self,
                                  key: &str,
                                  error_callback: impl Fn() -> ConundrumErrorVariant)
                                  -> ConundrumResult<ConundrumString> {
        if let Some(res) = self.data.get(key) {
            let item = res.value();
            match item {
                ParsedElement::Logic(l) => match l {
                    ConundrumLogicToken::String(s) => Some(s.clone()),
                    _ => None,
                },
                _ => None,
            }
        } else {
            None
        }.ok_or_else(error_callback)
    }

    pub fn get_string(&self, key: &str, error_msg: Option<&str>) -> ConundrumResult<ConundrumString> {
        if let Some(res) = self.data.get(key) {
            let item = res.value();
            match item {
                ParsedElement::Logic(l) => match l {
                    ConundrumLogicToken::String(s) => Some(s.clone()),
                    _ => None,
                },
                _ => None,
            }
        } else {
            None
        }.ok_or_else(|| {
             if let Some(msg) = error_msg {
                 ConundrumErrorVariant::InternalParserError(ConundrumError::from_message(msg))
             } else {
                 ConundrumErrorVariant::InternalParserError(ConundrumError::from_message(format!("Conundrum was looking for a _string_ and found a different value at the {} key", key).as_str()))
             }
         })
    }

    pub fn get_number(&self, key: &str, error_msg: Option<&str>) -> ConundrumResult<ConundrumNumber> {
        if let Some(res) = self.data.get(key) {
            let item = res.value();
            match item {
                ParsedElement::Logic(l) => match l {
                    ConundrumLogicToken::Number(n) => Some(n.clone()),
                    _ => None,
                },
                _ => None,
            }
        } else {
            None
        }.ok_or_else(|| {
             if let Some(msg) = error_msg {
                 ConundrumErrorVariant::InternalParserError(ConundrumError::from_message(msg))
             } else {
                 ConundrumErrorVariant::InternalParserError(ConundrumError::from_message(format!("Conundrum was looking for a _number_ and found a different value at the {} key", key).as_str()))
             }
         })
    }

    pub fn get_boolean(&self, key: &str, error_msg: Option<&str>) -> ConundrumResult<ConundrumBoolean> {
        if let Some(res) = self.data.get(key) {
            let item = res.value();
            match item {
                ParsedElement::Logic(l) => match l {
                    ConundrumLogicToken::Bool(b) => Some(b.clone()),
                    _ => None,
                },
                _ => None,
            }
        } else {
            None
        }.ok_or_else(|| {
             if let Some(msg) = error_msg {
                 ConundrumErrorVariant::InternalParserError(ConundrumError::from_message(msg))
             } else {
                 ConundrumErrorVariant::InternalParserError(ConundrumError::from_message(format!("Conundrum was looking for a _boolean_ and found a different value at the {} key", key).as_str()))
             }
         })
    }

    /// For when the sizable option if applied as a kv-pair, like
    /// `width="small"` instead of a boolean.
    pub fn get_sizable_option_at_key(&self, key: &str) -> ConundrumResult<SizableOption> {
        let s = self.get_string_with_err_cb(key, || {
            ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details(format!("The {} key is a `SizableOption`", key).as_str(), "This means it can only accept a limited set of options. See the `Sizable??` docs for more information."))
        })?;
        SizableOption::from_str(s.0.as_str())
    }
}
