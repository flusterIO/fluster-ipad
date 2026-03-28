use winnow::{ModalResult, Parser, stream::Stream};

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::parser_components::consume_white_space::consume_white_space,
};

pub fn white_space_delimited<'a, T>(mut parser: impl Fn(&mut ConundrumInput<'a>) -> ModalResult<T>)
                                    -> impl FnMut(&mut ConundrumInput<'a>) -> ModalResult<T> {
    move |input| {
        let start = input.input.checkpoint();
        consume_white_space(0..).parse_next(input).inspect_err(|_| {
                                                       input.input.reset(&start);
                                                   })?;
        let t = parser.parse_next(input).inspect_err(|_| {
                                             input.input.reset(&start);
                                         })?;

        consume_white_space(0..).parse_next(input).inspect_err(|_| {
                                                       input.input.reset(&start);
                                                   })?;

        Ok(t)
    }
}
