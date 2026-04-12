use winnow::{
    Parser,
    error::ParserError,
    stream::{AsChar, Stream, StreamIsPartial},
    token::take_while,
};

pub fn space_or_newline0<Input, Error>(input: &mut Input) -> Result<<Input as Stream>::Slice, Error>
    where Input: StreamIsPartial + Stream,
          <Input as Stream>::Token: AsChar + Copy,
          Error: ParserError<Input> {
    take_while(0.., |c| AsChar::is_space(c) || AsChar::is_newline(c)).parse_next(input)
}
