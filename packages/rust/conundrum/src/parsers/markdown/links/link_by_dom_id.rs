use winnow::{Parser, ascii::alphanumeric1, stream::Stream, token::literal};

use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    output::html::dom::dom_id::DOMId,
    parsers::{
        markdown::links::markdown_link_target::MarkdownLinkTarget,
        parser_components::consume_white_space::consume_linear_space,
    },
};

pub fn link_by_dom_id_target(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownLinkTarget> {
    let start = input.input.checkpoint();
    literal("id:").void().parse_next(input).inspect_err(|_| {
                                                input.input.reset(&start);
                                            })?;
    let target_id = alphanumeric1.map(|c: &str| DOMId(c.to_string())).parse_next(input).inspect_err(|_| {
                                                                                            input.input.reset(&start);
                                                                                        })?;
    consume_linear_space(0..).parse_next(input).inspect_err(|_| {
                                                    input.input.reset(&start);
                                                })?;
    ')'.parse_next(input).inspect_err(|_| {
                              input.input.reset(&start);
                          })?;
    Ok(MarkdownLinkTarget::DomId(target_id))
}
