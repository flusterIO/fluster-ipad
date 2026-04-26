use crate::{
    lang::runtime::{
        state::conundrum_error_variant::ConundrumModalResult,
        traits::{conundrum_input::ConundrumInput, indented_line_marker::IndentedLineMarker},
    },
    parsers::parsers_shared::shared_enums::line_terminator::{LineTerminator, indented_line_until_line_terminator},
};
use winnow::combinator::{alt, repeat};
use winnow::{Parser, stream::Range};

pub enum IndentedLineType<T> {
    EmptyLineContinuation,
    LineWithData(T),
}

pub fn join_indentend_line_types(data: Vec<IndentedLineType<(String, LineTerminator)>>) -> String {
    let mut s = String::from("");
    data.iter().for_each(|x| {
                   let concat_s = match x {
                       IndentedLineType::EmptyLineContinuation => "\n".to_string(),
                       IndentedLineType::LineWithData((s, t)) => format!("{}{}", s, t.string_representation()),
                   };
                   s += concat_s.as_str();
               });
    s
}

pub fn repeated_indented_lines(
    occurrences: impl Into<Range> + Clone)
    -> impl FnMut(&mut ConundrumInput) -> ConundrumModalResult<Vec<IndentedLineType<(String, LineTerminator)>>> {
    move |input| {
        let x: Vec<IndentedLineType<(String, LineTerminator)>> =
            repeat(occurrences.clone(),
                   alt((indented_line_until_line_terminator(1).map(|s: (String, LineTerminator)| {
                                                                  // let joined_string = x.iter().map(|(x, _)|
                                                                  // x.clone()).collect::<Vec<String>>().join("\n");
                                                                  IndentedLineType::LineWithData(s)
                                                              }),
                        '\n'.map(|_| IndentedLineType::EmptyLineContinuation)))).parse_next(input)?;
        Ok(x)
    }
}
