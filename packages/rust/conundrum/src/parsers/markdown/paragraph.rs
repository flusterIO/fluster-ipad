use std::cell::RefCell;

use fluster_core_utilities::core_types::component_constants::auto_inserted_component_name::AutoInsertedComponentName;
use serde::Serialize;
use winnow::{ModalResult, Parser, Stateful, combinator::alt, token::take_until};

use crate::lang::{
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
};

#[derive(Debug, Serialize)]
pub struct MarkdownParagraphResult {
    pub children: Vec<ParsedElement>,
}

impl MdxComponentResult for MarkdownParagraphResult {
    fn to_mdx_component(&self, res: &mut ParseState) -> String {
        let children_string = compile_elements(&self.children, res);
        format!("<{}>\n{}\n</{}>",
                AutoInsertedComponentName::AutoInsertedMarkdownParagraph,
                children_string.trim(),
                AutoInsertedComponentName::AutoInsertedMarkdownParagraph,)
    }
}

impl MarkdownParagraphResult {
    fn parse_input_string<'a>(input: &'a mut ConundrumInput<'a>) -> ModalResult<MarkdownParagraphResult> {
        let res = alt((take_until(1.., "```"), take_until(1.., "\n\n"))).parse_next(input)?;
        let state = input.state.borrow_mut();
        let mut new_input: Stateful<&str, RefCell<ParseState>> = get_conundrum_input(res, state.modifiers.clone());
        let children = parse_elements(&mut new_input)?;
        // apply_nested_parser_state(input, &new_input);
        Ok(MarkdownParagraphResult { children })
    }

    fn matches_first_char(char: char) -> bool {
        char == '$'
    }
}
