use winnow::{
    ModalResult, Parser,
    combinator::separated,
    stream::Stream,
    token::{literal, take_till},
};

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::{
        javascript::{
            function::{
                javascript_function::JavascriptFunction, javascript_function_parameter::javascript_function_parameter,
            },
            parsed_javascript_elements::ParsedJavascriptElement,
        },
        parser_components::white_space_delimited::white_space_delimited,
    },
};

pub fn javascript_arrow_function(input: &mut ConundrumInput) -> ModalResult<JavascriptFunction> {
    white_space_delimited(|input_inner| {
        let start = input_inner.input.checkpoint();
        let _ = '('.parse_next(input_inner).inspect_err(|_| {
                                                input_inner.input.reset(&start);
                                            })?;
        let function_arguments: Vec<ParsedJavascriptElement> =
            separated(0.., white_space_delimited(javascript_function_parameter), ',').parse_next(input_inner)
                                                                                     .inspect_err(|_| {
                                                                                         input_inner.input
                                                                                                    .reset(&start);
                                                                                     })?;
        let _ = ')'.parse_next(input_inner).inspect_err(|_| {
                                                input_inner.input.reset(&start);
                                            })?;

        let _ = white_space_delimited(|i| {
literal("=>").parse_next(i)
        }).parse_next(input_inner)
                                                            .inspect_err(|_| {
                                                                input_inner.input.reset(&start);
                                                            })?;

        let _ = '{'.parse_next(input_inner).inspect_err(|_| {
                                                input_inner.input.reset(&start);
                                            })?;

        // let js_string = compile_javascript...
        let buggy_function_body = take_till(0.., |c| c == '}').parse_next(input_inner)
                                                              .inspect_err(|_| {
                                                                  input_inner.input.reset(&start);
                                                              })?;

        println!("Here");
        let _ = '}'.parse_next(input_inner).inspect_err(|_| {
                                                input_inner.input.reset(&start);
                                            })?;

        Ok(JavascriptFunction { parameters: function_arguments,
                                javascript_body: buggy_function_body.to_string() })
    }).parse_next(input)
}

#[cfg(test)]
mod tests {
    use std::ops::Index;

    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_javascript_arrow_function() {
        let test_input = r#"(true, "My String", 3.14 ) => { 
const x = 3 / 4;
}"#;

        let mut test_data = wrap_test_conundrum_content(test_input);
        let res = javascript_arrow_function.parse_next(&mut test_data)
                                           .expect("Parses the function without throwing an error.");

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
                    ParsedJavascriptElement::Number(n) => n.value.as_float() == 3.14,
                    _ => false,
                },
                "Parses the proper argument for the third input");
    }
}
