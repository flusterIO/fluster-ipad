use winnow::{Parser, stream::Stream, token::literal};

use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    parsers::{
        markdown::links::markdown_link_target::MarkdownLinkTarget,
        parser_components::consume_white_space::consume_linear_space, parsers_shared::timestamp::timestamp,
    },
};

/// Matches the content inside the link, not the entire link.
/// It matches `[](<THIS PART HERE ONLY)>`
pub fn audio_timestamp_link_target(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownLinkTarget> {
    let start = input.input.checkpoint();
    literal("audio:").void().parse_next(input).inspect_err(|_| {
                                                   input.input.reset(&start);
                                               })?;
    let ts = timestamp.parse_next(input).inspect_err(|_| {
                                             input.input.reset(&start);
                                         })?;
    consume_linear_space(0..).parse_next(input).inspect_err(|_| {
                                                    input.input.reset(&start);
                                                })?;
    ')'.parse_next(input).inspect_err(|_| {
                              input.input.reset(&start);
                          })?;
    Ok(MarkdownLinkTarget::AudioTimestamp(ts))
}
