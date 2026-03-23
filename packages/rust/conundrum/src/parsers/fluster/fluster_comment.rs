use serde::Serialize;
use winnow::{ModalResult, Parser, ascii::till_line_ending, combinator::preceded, stream::Stream, token::literal};

use crate::{
    lang::runtime::traits::{conundrum_input::ConundrumInput, mdx_component_result::MdxComponentResult},
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
    parsers::parser_trait::ConundrumParser,
};

#[derive(Serialize, Debug)]
pub struct FlusterCommentResult {
    pub content: String,
}

impl MdxComponentResult for FlusterCommentResult {
    // No need to handle this implementation of the to_mdx_component method for the
    // ignore_all_parsers component since children will ignore it.
    fn to_mdx_component(&self, _: &mut MdxParsingResult) -> String {
        String::from("")
    }
}

impl ConundrumParser<FlusterCommentResult> for FlusterCommentResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ModalResult<FlusterCommentResult> {
        let start = input.input.checkpoint();
        let _ = literal("\n").parse_next(input).map_err(|e| {
                                                    println!("Error: {:#?}", e);
                                                    input.input.reset(&start);
                                                    e
                                                })?; // Consume the leading new line character
        //
        let line_content = preceded(literal("// FlusterComment: "), till_line_ending).parse_next(input)
                                                                                     .map_err(|e| {
                                                                                         println!("Error: {:#?}", e);
                                                                                         input.input.reset(&start);
                                                                                         e
                                                                                     })?;

        Ok(FlusterCommentResult { content: line_content.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '/'
    }
}
