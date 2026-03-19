use winnow::{
    Parser,
    ascii::line_ending,
    combinator::{alt, eof},
    error::ParserError,
};

pub fn line_ending_or_end_of_file<'a, Error: ParserError<&'a str>>() -> impl Parser<&'a str, &'a str, Error> {
    alt((line_ending, eof.value("")))
}
