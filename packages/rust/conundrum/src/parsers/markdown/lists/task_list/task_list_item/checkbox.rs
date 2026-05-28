use winnow::{Parser, stream::Stream};

use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    parsers::{
        markdown::lists::task_list::task_list_item::task_list_completion_indicator::TaskListCompletionToken,
        parser_trait::ConundrumParser,
    },
};

pub fn checkbox(input: &mut ConundrumInput) -> ConundrumModalResult<TaskListCompletionToken> {
    let start = input.input.checkpoint();
    '['.parse_next(input).inspect_err(|_| {
                              input.input.reset(&start);
                          })?;
    let token = TaskListCompletionToken::parse_input_string.parse_next(input).inspect_err(|_| {
                                                                                  input.input.reset(&start);
                                                                              })?;
    ']'.parse_next(input).inspect_err(|_| {
                              input.input.reset(&start);
                          })?;
    Ok(token)
}
