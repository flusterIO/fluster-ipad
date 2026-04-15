use winnow::{Parser, stream::Stream};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            state::conundrum_error_variant::ConundrumModalResult,
            traits::conundrum_input::{ConundrumInput, get_conundrum_input},
        },
    },
    parsers::{
        javascript::{
            javascript_number::JavascriptNumberResult, javascript_parser_trait::JavascriptParser,
            object::javascript_key_value_pair::JavascriptObjectKeyValuePair,
            parsed_javascript_elements::ParsedJavascriptElement,
        },
        parser_components::white_space_delimited::white_space_delimited,
        react::parser_components::jsx_properties::{
            jsx_curly_bracket_wrapped_property::jsx_curly_bracket_wrapped_property, jsx_property::JsxPropertyParser,
        },
    },
};

pub struct JsxNumberPropertyResult {}

impl JsxPropertyParser for JsxNumberPropertyResult {
    fn parse_jsx_property(input: &mut ConundrumInput) -> ConundrumModalResult<JavascriptObjectKeyValuePair> {
        let start = input.input.checkpoint();
        let (key, bracketed_content) = jsx_curly_bracket_wrapped_property.parse_next(input)
                                                                         .inspect_err(|_| {
                                                                             input.input.reset(&start);
                                                                         })?;

        let state = input.state.borrow();

        let mut new_input = get_conundrum_input(&bracketed_content, state.modifiers.clone(), state.ui_params.clone());

        let n = white_space_delimited(JavascriptNumberResult::parse_javascript).parse_next(&mut new_input)
                                                                               .inspect_err(|_| {
                                                                                   input.input.reset(&start);
                                                                               })?;

        Ok(JavascriptObjectKeyValuePair { key,
                                          value:
                                              Box::new(ParsedElement::Javascript(ParsedJavascriptElement::Number(n))) })
    }
}
