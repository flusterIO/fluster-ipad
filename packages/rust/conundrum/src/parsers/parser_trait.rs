use winnow::ModalResult;

pub trait ConundrumParser<T> {
    fn parse_input_string(input: &mut &str) -> ModalResult<T>;
    fn matches_first_char(char: char) -> bool;
}
