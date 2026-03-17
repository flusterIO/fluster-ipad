use winnow::combinator::{dispatch, opt, peek};
use winnow::token::take;
use winnow::{
    ModalResult, Parser,
    combinator::{alt, repeat},
    token::any,
};

use crate::{
    lang::elements::{parsed_code_block::ParsedCodeBlock, parsed_elements::ParsedElement},
    parsers::{
        fluster::{inline_citation::ParsedCitation, note_link::ParsedOutgoingNoteLink, tag::ParsedTag},
        markdown::parser_trait::ConundrumParser,
    },
};

pub fn parse_conundrum_string(input: &mut &str) -> ModalResult<Vec<ParsedElement>> {
    repeat(0..,
           dispatch! {peek(take(1usize));
               "`" => |x: &mut &str| {
                   opt(ParsedCodeBlock::parse_input_string.map(ParsedElement::CodeBlock)).parse_next(x).map_or_else(|z| {
                        Err(z)
                   }, |y| {
                        y.map(|w| {
                        Ok(w)
                        }).unwrap_or_else(|| {
                           any.map(|c: char| ParsedElement::Text(c.to_string())).parse_next(x)
                        })
                   })
               },
               "[" => |x: &mut &str| {
                   let res = opt(alt((
                       ParsedCitation::parse_input_string.map(ParsedElement::Citation),
                       ParsedOutgoingNoteLink::parse_input_string.map(ParsedElement::OutgoingNoteLink),
                       ParsedTag::parse_input_string.map(ParsedElement::Tag)
                   )))
                       .parse_next(x)
                       .or_else(|_| {
                           any.map(|c: char| ParsedElement::Text(c.to_string()))
                               .parse_next(x)
                               .map(Some)
                   })?;
                   match res {
                       Some(element) => Ok(element),
                       None => {
                           any.map(|c: char| ParsedElement::Text(c.to_string())).parse_next(x)
                       }
                   }
               },
               _ => {
                   any.map(|c: char| ParsedElement::Text(c.to_string()))
               },
           }).parse_next(input)
}
