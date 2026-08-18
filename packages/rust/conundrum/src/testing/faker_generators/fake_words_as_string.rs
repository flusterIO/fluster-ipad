use std::{ops::Range, path::PathBuf};

use fake::{Fake, faker::lorem::en::Word, faker::lorem::en::Words};

pub fn fake_words_as_string(words: Range<usize>) -> String {
    let words = Words(words);
    let res: Vec<String> = words.fake();
    res.join(" ")
}

pub fn fake_words_as_optional_string(words: Range<usize>) -> Option<String> {
    let words = Words(words);
    let res: Vec<String> = words.fake();
    Some(res.join(" "))
}

pub fn fake_file_path(words: Range<usize>) -> String {
    let mut x = PathBuf::new();
    for _ in words {
        let w = Word();
        let s: String = w.fake();
        x = x.join(s);
    }
    x.to_str().expect("Must deserialize to string").to_string()
}
