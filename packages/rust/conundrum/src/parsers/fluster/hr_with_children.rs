use serde::Serialize;
use winnow::{
    ModalResult, Parser,
    combinator::delimited,
    stream::Stream,
    token::{literal, take_until},
};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            compile_conundrum::compile_elements,
            parse_conundrum_string::parse_elements,
            state::parse_state::ParseState,
            traits::{
                conundrum_input::{ConundrumInput, get_conundrum_input},
                mdx_component_result::MdxComponentResult,
            },
        },
    },
    parsers::parser_trait::ConundrumParser,
};

#[derive(Serialize, Debug)]
pub struct HrWithChildrenResult {
    pub children: Vec<ParsedElement>,
}

impl MdxComponentResult for HrWithChildrenResult {
    // No need to handle this implementation of the to_mdx_component method for the
    // ignore_all_parsers component since children will ignore it.
    fn to_mdx_component(&self, res: &mut ParseState) -> String {
        let children_string = compile_elements(&self.children, res);

        format!("<Hr>{}</Hr>", children_string)
    }
}

impl ConundrumParser<HrWithChildrenResult> for HrWithChildrenResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ModalResult<HrWithChildrenResult> {
        let start = input.input.checkpoint();
        let _ = literal("\n").parse_next(input).map_err(|e| {
                                                    input.input.reset(&start);
                                                    e
                                                })?; // Consume the leading new line character
        let res = delimited(literal("--- "),
                            take_until(1.., " ---").verify(|s: &str| !s.contains("\n")),
                            literal(" ---")).parse_next(input)
                                            .map_err(|e| {
                                                input.input.reset(&start);
                                                e
                                            })?;

        let state = input.state.borrow_mut();

        let mut new_input = get_conundrum_input(res, state.modifiers.clone());
        let elements = parse_elements(&mut new_input)?;
        // WITH_WIFI: Figure out how to call this without throwing reference errors.
        // apply_nested_parser_state(input, &new_input);

        Ok(HrWithChildrenResult { children: elements })
    }

    fn matches_first_char(char: char) -> bool {
        char == '['
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_hr_with_children() {
        let test_content = "--- My Hr with children ---";
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res = HrWithChildrenResult::parse_input_string(&mut test_data).expect("Parses hr with children without throwing an error.");
        let state = test_data.state.borrow();
        let mut res_data = state.clone();
        let child = res.to_mdx_component(&mut res_data);
        assert_snapshot!(child)
        // assert_eq!(result, 4);
    }

    #[test]
    fn parses_hr_with_children_should_fail_with_new_line() {
        let test_content = r#"--- My Hr with 
            children ---"#;
        let mut test_data = wrap_test_conundrum_content(test_content);
        let _ = HrWithChildrenResult::parse_input_string(&mut test_data).expect_err("HR with children parser fails with a new line as it should.");
        // assert_eq!(result, 4);
    }
}
