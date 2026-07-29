use std::ops::Range;

use fake::{Fake, faker::lorem::en::Words};

pub fn fake_words_as_string(words: Range<usize>) -> String {
    let words = Words(words);
    let res: Vec<String> = words.fake();
    res.join(" ")
}
