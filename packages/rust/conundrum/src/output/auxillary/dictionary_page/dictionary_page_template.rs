use crate::output::{
    auxillary::dictionary_page::dictionary_letter::DictionaryLetter,
    parsing_result::dictionary_result::DictionaryEntryResult,
};

pub struct DictionaryPageTemplate {
    letters: Vec<DictionaryLetter>,
    entries: Vec<DictionaryEntryResult>,
}
