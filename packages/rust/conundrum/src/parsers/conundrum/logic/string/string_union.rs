use serde::Serialize;
use winnow::Parser;

use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    parsers::{
        javascript::object::javascript_key_value_pair::JavascriptObjectKeyValuePair,
        react::parser_components::jsx_properties::{
            jsx_property::JsxPropertyParser, string_property::JsxStringPropertyResult,
        },
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct StringUnion {
    pub opts: Vec<String>,
    pub value: String,
}

impl StringUnion {
    fn parse_jsx_property(options: Vec<String>)
                          -> impl Fn(&mut ConundrumInput) -> ConundrumModalResult<JavascriptObjectKeyValuePair> {
        move |input: &mut ConundrumInput| -> ConundrumModalResult<JavascriptObjectKeyValuePair> {
            JsxStringPropertyResult::parse_jsx_property.verify(|s| {
                                                           if let Ok(res_as_string) = s.value.get_string() {
                                                               let is_allowed = options.contains(&res_as_string.0);
                                                               if is_allowed {
                                                                   true
                                                               } else {
                                                                   false
                                                               }
                                                           } else {
                                                               false
                                                           }
                                                       })
                                                       .parse_next(input)
        }
    }
}
