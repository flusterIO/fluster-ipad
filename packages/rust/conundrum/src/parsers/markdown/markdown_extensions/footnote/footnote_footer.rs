use serde::Serialize;
use tokio::io::repeat;
use winnow::{
    Parser,
    ascii::{space1, till_line_ending},
    stream::Stream,
};

use crate::{
    lang::{
        lib::ui::ui_types::children::Children,
        runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    },
    parsers::{markdown::markdown_extensions::footnote::footnote_anchor, parser_trait::ConundrumParser},
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct FootNoteFooter {
    pub idx: u32,
    pub content: Children,
}

// pub fn tab_or_space_indent(input: &mut ConundrumInput) ->
// ConundrumModalResult<&str> {

// }

// pub fn indented_line(input: &mut ConundrumInput) ->
// ConundrumModalResult<&str> {
//
// }

// impl ConundrumParser for FootNoteFooter {
//     fn parse_input_string(input: &mut
// crate::lang::runtime::traits::conundrum_input::ConundrumInput)
// -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<T>
// {         let start = input.input.checkpoint();
//         let a =
// footnote_anchor::FootNoteAnchor::parse_input_string.parse_next(input).
// inspect_err(|_| {
// input.input.reset(&start);
// })?;

//         let _ = space1.parse_next(input).inspect_err(|_| {
//                                              input.input.reset(&start);
//                                          })?;

//         let content = till_line_ending.parse_next(input).inspect_err(|_| {
//
// input.input.reset(&start);
// })?;

//         let repeated_lines =
//     }

//     fn matches_first_char(char: char) -> bool {
//         todo!()
//     }
// }
