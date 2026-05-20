use winnow::{
    Parser,
    error::{ContextError, ErrMode},
    stream::{AsChar, Stream},
    token::take_till,
};

use crate::{
    lang::runtime::{
        state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
        },
        traits::conundrum_input::ConundrumInput,
    },
    parsers::{
        markdown::links::markdown_link_target::MarkdownLinkTarget,
        parser_components::consume_white_space::consume_linear_space,
    },
};

pub fn general_link_target(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownLinkTarget> {
    let start = input.input.checkpoint();
    let x = take_till(1.., |c| c == ')' || AsChar::is_newline(c) || AsChar::is_space(c)).parse_next(input)
                                                                                        .map_err(|_: ContextError| {
                                                                                            input.input.reset(&start);
                                                                                            ErrMode::Backtrack(
    ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Not a general markdown link")))
                                                                                        })?;

    consume_linear_space(0..).parse_next(input).inspect_err(|_| {
                                                    input.input.reset(&start);
                                                })?;
    ')'.parse_next(input).inspect_err(|_| {
                              input.input.reset(&start);
                          })?;
    Ok(MarkdownLinkTarget::Url(x.to_string()))
}
