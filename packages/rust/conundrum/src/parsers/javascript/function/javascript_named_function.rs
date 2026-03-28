use winnow::{ModalResult, Parser, combinator::separated, stream::Stream, token::take_till};

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::{
        javascript::{
            function::{
                javascript_function::JavascriptFunction,
                javascript_function_parameter::{self, javascript_function_parameter},
            },
            parsed_javascript_elements::ParsedJavascriptElement,
        },
        parser_components::{consume_white_space::consume_white_space, white_space_delimited::white_space_delimited},
    },
};

pub fn javascript_function(input: &mut ConundrumInput) -> ModalResult<JavascriptFunction> {
    let start = input.input.checkpoint();
    let _ = '('.parse_next(input).inspect_err(|_| {
                                      input.input.reset(&start);
                                  })?;

    consume_white_space(0..).parse_next(input).inspect_err(|_| {
                                                   input.input.reset(&start);
                                               })?;

    let function_arguments: Vec<ParsedJavascriptElement> =
        separated(0.., white_space_delimited(javascript_function_parameter), ',').parse_next(input)
                                                                                 .inspect_err(|_| {
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
    let function_body = take_till(0.., |c| c == '}').parse_next(input).inspect_err(|_| {
                                                                           input.input.reset(&start);
                                                                       })?;

    let _ = '}'.parse_next(input).inspect_err(|_| {
                                      input.input.reset(&start);
                                  })?;

    Ok(JavascriptFunction { parameters: function_arguments,
                            javascript_body: function_body.to_string() })
}

#[cfg(test)]
mod tests {
    use std::ops::Index;

    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_named_function() {
        let test_input = r#"(true,
        "My String",
        3.14 ) {
    const x = y / z;
}"#;
        let mut test_data = wrap_test_conundrum_content(test_input);
        let res =
            javascript_function.parse_next(&mut test_data).expect("Parses the function without throwing an error.");

        assert!(test_data.input.is_empty(), "Consumes the entire function.");

        assert!(res.parameters.len() == 3, "Returns the proper number of parameters");

        assert!(match res.parameters.index(0) {
                    ParsedJavascriptElement::Boolean(b) => b.value,
                    _ => false,
                },
                "Parses the proper argument for the first input");

        assert!(match res.parameters.index(1) {
                    ParsedJavascriptElement::String(s) => s.value == "My String",
                    _ => false,
                },
                "Parses the proper argument for the second input");

        assert!(match res.parameters.index(2) {
                    #[allow(clippy::approx_constant)]
                    ParsedJavascriptElement::Number(n) => n.value.as_float() == 3.14,
                    _ => false,
                },
                "Parses the proper argument for the third input");
    }
}
