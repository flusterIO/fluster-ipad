use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    parsers::{markdown::paragraph::MarkdownParagraphResult, parser_trait::ConundrumParser},
};
use winnow::{Parser, combinator::repeat};

pub struct ConundrumDocument(Vec<MarkdownParagraphResult>);

impl ConundrumDocument {
    pub fn parse_input(input: &mut ConundrumInput) -> ConundrumModalResult<Self> {
        let r: Vec<MarkdownParagraphResult> =
            repeat(0.., MarkdownParagraphResult::parse_input_string).parse_next(input)?;
        Ok(ConundrumDocument(r))
    }
}
