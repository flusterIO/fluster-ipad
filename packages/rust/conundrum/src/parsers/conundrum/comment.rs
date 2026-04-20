use serde::Serialize;
use winnow::{Parser, ascii::till_line_ending, combinator::preceded, stream::Stream, token::literal};

use crate::{
    lang::runtime::{
        state::{conundrum_error_variant::ConundrumModalResult, parse_state::ConundrumCompileTarget},
        traits::{
            conundrum_input::{ArcState, ConundrumInput},
            fluster_component_result::ConundrumComponentResult,
            mdx_component_result::MdxComponentResult,
            plain_text_component_result::PlainTextComponentResult,
        },
    },
    parsers::parser_trait::ConundrumParser,
};

#[typeshare::typeshare]
#[derive(Serialize, Debug, Clone)]
pub struct ConundrumCommentResult {
    pub content: String,
}

impl PlainTextComponentResult for ConundrumCommentResult {
    fn to_plain_text(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}

impl ConundrumComponentResult for ConundrumCommentResult {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.compile_target == ConundrumCompileTarget::PlainText {
            drop(state);
            self.to_plain_text(res)
        } else {
            drop(state);
            self.to_mdx_component(res)
        }
    }
}

impl MdxComponentResult for ConundrumCommentResult {
    // No need to handle this implementation of the to_mdx_component method for the
    // ignore_all_parsers component since children will ignore it.
    fn to_mdx_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}

impl ConundrumParser<ConundrumCommentResult> for ConundrumCommentResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumCommentResult> {
        let start = input.input.checkpoint();
        let line_content = preceded(literal("//**: "), till_line_ending).parse_next(input)
                                                                        .inspect_err(|_| {
                                                                            input.input.reset(&start);
                                                                        })?;

        Ok(ConundrumCommentResult { content: line_content.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '/'
    }
}
