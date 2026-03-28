use winnow::{ModalResult, Parser, combinator::separated, stream::Stream, token::take_till};

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::{
        javascript::{
            function::{
                javascript_function::JavascriptFunction, javascript_function_parameter::javascript_function_parameter,
            },
            parsed_javascript_elements::ParsedJavascriptElement,
        },
        parser_components::consume_white_space::consume_white_space,
    },
};

pub fn javascript_function(input: &mut ConundrumInput) -> ModalResult<JavascriptFunction> {
    let start = input.input.checkpoint();
    consume_white_space(0..).parse_next(input).inspect_err(|_| {
                                                   input.input.reset(&start);
                                               })?;
    let _ = '('.parse_next(input).inspect_err(|_| {
                                      input.input.reset(&start);
                                  })?;

    consume_white_space(0..).parse_next(input).inspect_err(|_| {
                                                   input.input.reset(&start);
                                               })?;

    let function_arguments: Vec<ParsedJavascriptElement> =
        separated(0.., javascript_function_parameter, ',').parse_next(input).inspect_err(|_| {
                                                                                 input.input.reset(&start);
                                                                             })?;

    consume_white_space(0..).parse_next(input).inspect_err(|_| {
                                                   input.input.reset(&start);
                                               })?;
    let _ = ')'.parse_next(input).inspect_err(|_| {
                                      input.input.reset(&start);
                                  })?;

    consume_white_space(0..).parse_next(input).inspect_err(|_| {
                                                   input.input.reset(&start);
                                               })?;

    let _ = '{'.parse_next(input).inspect_err(|_| {
                                      input.input.reset(&start);
                                  })?;

    // BUG: This will break the parser when a function contains a nested '}'. This
    // will do for now as most users won't write javascript functions outside of
    // code blocks, but this will eventually break the parser and allow React
    // components to sneak into AI input and the like.
    let function_body = take_till(0.., |c| c != '}').parse_next(input).inspect_err(|_| {
                                                                           input.input.reset(&start);
                                                                       })?;

    let _ = '}'.parse_next(input).inspect_err(|_| {
                                      input.input.reset(&start);
                                  })?;

    Ok(JavascriptFunction { parameters: function_arguments,
                            javascript_body: function_body.to_string() })
}
