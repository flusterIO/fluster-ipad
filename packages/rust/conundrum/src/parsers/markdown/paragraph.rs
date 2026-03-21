use fluster_core_utilities::core_types::component_constants::auto_inserted_component_name::AutoInsertedComponentName;
use serde::Serialize;
use typeshare::typeshare;
use winnow::{ModalResult, Parser, combinator::alt, token::take_until};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            compile_conundrum::compile_elements,
            parse_conundrum_string::parse_elements,
            traits::{
                conundrum_input::{ConundrumInput, get_conundrum_input},
                mdx_component_result::MdxComponentResult,
            },
        },
    },
    parsers::parser_trait::ConundrumParser,
};

#[derive(Debug, Serialize)]
pub struct MarkdownParagraphResult {
    pub children: Vec<ParsedElement>,
}

impl MdxComponentResult for MarkdownParagraphResult {
    fn to_mdx_component(&self,
                        res: &mut crate::output::parsing_result::mdx_parsing_result::MdxParsingResult)
                        -> String {
        let children_string = compile_elements(&self.children, res);
        format!("<{}>\n{}\n</{}>",
                AutoInsertedComponentName::AutoInsertedMarkdownParagraph,
                children_string.trim(),
                AutoInsertedComponentName::AutoInsertedMarkdownParagraph,)
    }
}

impl ConundrumParser<MarkdownParagraphResult> for MarkdownParagraphResult {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ModalResult<MarkdownParagraphResult> {
        let res = alt((take_until(1.., "```"), take_until(1.., "\n\n"))).parse_next(input)?;
        let mut new_input = get_conundrum_input(res);
        let children = parse_elements(&mut new_input)?;
        Ok(MarkdownParagraphResult { children })
    }

    fn matches_first_char(char: char) -> bool {
        char == '$'
    }
}
