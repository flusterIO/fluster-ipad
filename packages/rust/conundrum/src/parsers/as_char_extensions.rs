use winnow::stream::AsChar;

pub fn is_space_or_newline(c: char) -> bool {
    AsChar::is_space(c) || AsChar::is_newline(c)
}
