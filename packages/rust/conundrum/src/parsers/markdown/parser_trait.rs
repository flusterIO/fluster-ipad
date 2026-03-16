use nom::IResult;

pub trait ConundrumParser<T> {
    fn parse_input_string(content: &str) -> IResult<&str, T>;
}
