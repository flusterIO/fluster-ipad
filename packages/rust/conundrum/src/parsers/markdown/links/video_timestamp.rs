use winnow::{Parser, ascii::alphanumeric1, stream::Stream, token::literal};

use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    output::html::dom::dom_id::DOMId,
    parsers::{parser_components::consume_white_space::consume_linear_space, parser_trait::ConundrumParser},
};

pub struct VideoTimestampLink {
    pub target_id: DOMId,
}

impl ConundrumParser<VideoTimestampLink> for VideoTimestampLink {
    /// Matches the content inside the link, not the entire link.
    /// It matches `[](<THIS PART HERE ONLY)>`
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<VideoTimestampLink> {
        let start = input.input.checkpoint();
        literal("id:").void().parse_next(input).inspect_err(|_| {
                                                    input.input.reset(&start);
                                                })?;
        let target_id =
            alphanumeric1.map(|c: &str| DOMId(c.to_string())).parse_next(input).inspect_err(|_| {
                                                                                    input.input.reset(&start);
                                                                                })?;
        consume_linear_space(0..).parse_next(input).inspect_err(|_| {
                                                        input.input.reset(&start);
                                                    })?;
        ')'.parse_next(input).inspect_err(|_| {
                                  input.input.reset(&start);
                              })?;
        Ok(VideoTimestampLink { target_id })
    }

    fn matches_first_char(char: char) -> bool {
        true
    }
}
