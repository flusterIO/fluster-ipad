use serde::Serialize;
use winnow::{Parser, combinator::alt, token::take_until};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            compile_conundrum::compile_elements,
            parse_conundrum_string::parse_elements,
            state::{
                conundrum_error_variant::ConundrumModalResult,
                parse_state::{ConundrumModifier, ParseState},
            },
            traits::{
                conundrum_input::{ConundrumInput, get_conundrum_input},
                fluster_component_result::ConundrumComponentResult,
                mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::auto_inserted_component_name::AutoInsertedComponentName,
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct MarkdownParagraphResult {
    pub children: Vec<ParsedElement>,
}

impl PlainTextComponentResult for MarkdownParagraphResult {
    fn to_plain_text(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        compile_elements(&self.children, res)
    }
}

impl ConundrumComponentResult for MarkdownParagraphResult {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MdxComponentResult for MarkdownParagraphResult {
    fn to_mdx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        let children_string = compile_elements(&self.children, res)?;
        Ok(format!("<{}>\n{}\n</{}>",
                   AutoInsertedComponentName::AutoInsertedMarkdownParagraph,
                   children_string.trim(),
                   AutoInsertedComponentName::AutoInsertedMarkdownParagraph,))
    }
}

impl MarkdownParagraphResult {
    fn parse_input_string<'a>(input: &'a mut ConundrumInput<'a>) -> ConundrumModalResult<MarkdownParagraphResult> {
        let res = alt((take_until(1.., "```"), take_until(1.., "\n\n"))).parse_next(input)?;
        let state = input.state.borrow_mut();
        let mut new_input = get_conundrum_input(res, state.modifiers.clone(), state.ui_params.clone());
        let children = parse_elements(&mut new_input)?;
        // apply_nested_parser_state(input, &new_input);
        Ok(MarkdownParagraphResult { children })
    }

    fn matches_first_char(char: char) -> bool {
        char == '$'
    }
}
