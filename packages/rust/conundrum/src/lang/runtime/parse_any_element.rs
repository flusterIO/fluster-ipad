use winnow::combinator::{dispatch, peek};
use winnow::{
    ModalResult, Parser,
    combinator::{alt, repeat},
    token::any,
};

use crate::{
    lang::elements::{parsed_code_block::ParsedCodeBlock, parsed_elements::ParsedElement},
    parsers::{
        fluster::{
            inline_citation::ParsedCitation, note_link::ParsedOutgoingNoteLink, tag::ParsedTag,
        },
        markdown::parser_trait::ConundrumParser,
    },
};

pub fn parse_conundrum_string(input: &mut &str) -> ModalResult<Vec<ParsedElement>> {
    repeat(
        0..,
        dispatch! {peek(any);
            '`' => ParsedCodeBlock::parse_input_string.map(ParsedElement::CodeBlock),
            '[' => alt((
                ParsedCitation::parse_input_string.map(ParsedElement::Citation),
                ParsedOutgoingNoteLink::parse_input_string.map(ParsedElement::OutgoingNoteLink),
            )),
            '#' => ParsedTag::parse_input_string.map(ParsedElement::Tag),
            _ => any.map(|c: char| ParsedElement::Text(c.to_string())),
        },
    )
    .parse_next(input)
}
