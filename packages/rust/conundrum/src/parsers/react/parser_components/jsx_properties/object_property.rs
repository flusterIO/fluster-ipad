use winnow::{ModalResult, Parser, stream::Stream};

use crate::{
    lang::runtime::traits::conundrum_input::{ConundrumInput, get_conundrum_input},
    parsers::{
        javascript::{javascript_parser_trait::JavascriptParser, object::javascript_object::JavascriptObjectResult},
        react::parser_components::jsx_properties::{
            jsx_curly_bracket_wrapped_property::jsx_curly_bracket_wrapped_property, jsx_property::JsxPropertyParser,
        },
    },
};

pub struct JsxObjectPropertyResult {
    pub object: JavascriptObjectResult,
    pub key: String,
}

impl JsxPropertyParser<JsxObjectPropertyResult> for JsxObjectPropertyResult {
    fn parse_jsx_property(input: &mut ConundrumInput) -> ModalResult<JsxObjectPropertyResult> {
        let start = input.input.checkpoint();
        let (key, wrapped_content) = jsx_curly_bracket_wrapped_property.parse_next(input).inspect_err(|_| {
                                                                                              input.input.reset(&start);
                                                                                          })?;

        let state = input.state.borrow();

        let mut new_input = get_conundrum_input(&wrapped_content, state.modifiers.clone());

        let object_data = JavascriptObjectResult::parse_javascript(&mut new_input)?;

        Ok(JsxObjectPropertyResult { object: object_data,
                                     key })
    }
}
