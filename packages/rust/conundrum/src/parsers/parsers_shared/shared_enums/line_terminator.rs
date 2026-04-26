use crate::{
    lang::runtime::{
        state::conundrum_error_variant::ConundrumModalResult,
        traits::{conundrum_input::ConundrumInput, indented_line_marker::IndentedLineMarker},
    },
    parsers::parsers_shared::indentation_handling::indentation_count::indentation,
};
use winnow::{
    Parser,
    combinator::{alt, eof, preceded, repeat, repeat_till},
    stream::Range,
    token::{literal, take},
};

pub fn markdown_line_terminator(input: &mut ConundrumInput) -> ConundrumModalResult<LineTerminator> {
    alt(('\n'.map(|_| LineTerminator::Newline),
         literal("\r\n").map(|_| LineTerminator::Ctrlf),
         alt((literal("  \r\n"), literal("  \n"))).map(|_| LineTerminator::DoubleSpaceNewLine),
         eof.map(|_| LineTerminator::Eof))).parse_next(input)
}

pub enum LineTerminator {
    Newline,
    /// That f--king windows new line.
    Ctrlf,
    DoubleSpaceNewLine,
    DoubleNewLine,
    Eof,
}

impl IndentedLineMarker for LineTerminator {
    fn string_representation(&self) -> String {
        match self {
            Self::Newline => "\n".to_string(),
            Self::Ctrlf => "\r\n".to_string(),
            Self::DoubleSpaceNewLine => "  \n".to_string(),
            Self::DoubleNewLine => "\n\n".to_string(),
            Self::Eof => "".to_string(),
        }
    }

    fn should_continue_indented_line_iter(&self) -> bool {
        match self {
            Self::Newline => true,
            Self::Ctrlf => true,
            Self::DoubleSpaceNewLine => false,
            Self::DoubleNewLine => false,
            Self::Eof => false,
        }
    }
}

pub fn until_line_terminator<'a, T>(
    occurrences: impl Into<Range> + Clone,
    mut parser: impl Fn(&mut ConundrumInput<'a>) -> ConundrumModalResult<T>)
    -> impl FnMut(&mut ConundrumInput<'a>) -> ConundrumModalResult<(Vec<T>, LineTerminator)> {
    move |input: &mut ConundrumInput<'a>| {
        let (res, terminator): (Vec<T>, LineTerminator) =
            repeat_till(occurrences.clone(),
                        |nested_input: &mut ConundrumInput<'a>| parser.parse_next(nested_input),
                        markdown_line_terminator).parse_next(input)?;
        Ok((res, terminator))
    }
}

pub fn indented_line_until_line_terminator<'a>(
    indententation_depth: impl Into<Range> + Clone)
    -> impl FnMut(&mut ConundrumInput<'a>) -> ConundrumModalResult<(String, LineTerminator)> {
    move |input: &mut ConundrumInput<'a>| {
        let (res, terminator): (String, LineTerminator) =
            preceded(|nested_input: &mut ConundrumInput<'a>| {
                         let _: Vec<String> =
                             repeat(indententation_depth.clone(), indentation).parse_next(nested_input)?;
                         Ok(())
                     },
                     |nested_input: &mut ConundrumInput<'a>| {
                         let (res, terminator): (Vec<&str>, LineTerminator) =
                             until_line_terminator(1.., |nested_nested_input: &mut ConundrumInput<'a>| {
                                 take(1usize).parse_next(nested_nested_input)
                             }).parse_next(nested_input)?;
                         Ok((res.join(""), terminator))
                     }).parse_next(input)?;

        Ok((res, terminator))
    }
}
