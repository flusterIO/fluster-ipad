use winnow::Parser;
use winnow::combinator::alt;

use crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult;
use crate::lang::runtime::traits::conundrum_input::ConundrumInput;
use crate::parsers::javascript::javascript_boolean::JavascriptBooleanResult;
use crate::parsers::javascript::javascript_number::JavascriptNumberResult;
use crate::parsers::javascript::javascript_parser_trait::JavascriptParser;
use crate::parsers::javascript::string::javascript_string::single_or_double_quoted_string;
use crate::parsers::react::parser_components::jsx_properties::jsx_property_key::jsx_property_key;

pub fn javascript_object_key(input: &mut ConundrumInput) -> ConundrumModalResult<String> {
    let k = alt((jsx_property_key,
                 single_or_double_quoted_string.map(|s| s.value),
                 JavascriptNumberResult::parse_javascript.map(|n| format!("{:?}", n.value)),
                 JavascriptBooleanResult::parse_javascript.map(|n| {
                                                              if n.value {
                                                                  String::from("true")
                                                              } else {
                                                                  String::from("false")
                                                              }
                                                          }))).parse_next(input)?;

    Ok(k)
}
