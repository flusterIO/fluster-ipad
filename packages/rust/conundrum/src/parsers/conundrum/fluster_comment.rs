use serde::Serialize;
use winnow::{ModalResult, Parser, ascii::till_line_ending, combinator::preceded, stream::Stream, token::literal};

use crate::{
    lang::runtime::{
        state::parse_state::{ConundrumModifier, ParseState},
        traits::{
            conundrum_input::ConundrumInput, fluster_component_result::ConundrumComponentResult,
            mdx_component_result::MdxComponentResult, plain_text_component_result::PlainTextComponentResult,
        },
    },
    parsers::parser_trait::ConundrumParser,
};

#[derive(Serialize, Debug)]
pub struct ConundrumCommentResult {
    pub content: String,
}

impl PlainTextComponentResult for ConundrumCommentResult {
    fn to_plain_text(&self, _: &mut ParseState) -> String {
        String::from("")
    }
}

impl ConundrumComponentResult for ConundrumCommentResult {
    fn to_conundrum_component(&self, res: &mut ParseState) -> String {
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MdxComponentResult for ConundrumCommentResult {
    // No need to handle this implementation of the to_mdx_component method for the
    // ignore_all_parsers component since children will ignore it.
    fn to_mdx_component(&self, _: &mut ParseState) -> String {
        String::from("")
    }
}

impl ConundrumParser<ConundrumCommentResult> for ConundrumCommentResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ModalResult<ConundrumCommentResult> {
        let start = input.input.checkpoint();
        let line_content = preceded(literal("// FlusterComment: "), till_line_ending).parse_next(input)
                                                                                     .inspect_err(|_| {
                                                                                         input.input.reset(&start);
                                                                                     })?;

        Ok(ConundrumCommentResult { content: line_content.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '/'
    }
}
