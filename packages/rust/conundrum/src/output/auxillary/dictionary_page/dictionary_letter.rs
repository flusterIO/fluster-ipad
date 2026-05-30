use std::fmt::Display;

/// TODO: Implement the safer unicode comparison for internationalization here.
#[derive(Clone)]
pub struct DictionaryLetter(char);

impl Display for DictionaryLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
