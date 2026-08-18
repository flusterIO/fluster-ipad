use std::ops::Range;

use fake::{Fake, faker::lorem::en::Paragraphs};

pub fn fake_cdrm_content(paragraphs: Range<usize>) -> String {
    let paragraphs = Paragraphs(paragraphs);
    let res: Vec<String> = paragraphs.fake();
    res.join("\n\n")
}
