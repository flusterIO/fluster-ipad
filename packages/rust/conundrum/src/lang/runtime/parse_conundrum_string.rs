use fluster_core_utilities::core_types::fluster_error::{FlusterError, FlusterResult};
use winnow::combinator::{dispatch, peek};
use winnow::token::take;
use winnow::{
    ModalResult, Parser,
    combinator::{alt, repeat},
    token::any,
};

use crate::parsers::fluster::docs::ParsedInspectionRequest;
use crate::{
    lang::elements::{parsed_code_block::ParsedCodeBlock, parsed_elements::ParsedElement},
    parsers::{
        fluster::{inline_citation::ParsedCitation, note_link::ParsedOutgoingNoteLink, tag::ParsedTag},
        markdown::parser_trait::ConundrumParser,
    },
};

pub fn parse_conundrum_string(input: &mut &str) -> FlusterResult<Vec<ParsedElement>> {
    repeat(0..,
           dispatch! {peek(take(1usize));
               "`" => |x: &mut &str| {
                   alt((
                       ParsedCodeBlock::parse_input_string.map(ParsedElement::ParsedCodeBlock),
                       any.map(|c: char| ParsedElement::Text(c.to_string()))
                   )).parse_next(x)
               },
               "[" => |x: &mut &str| {
                   alt((
                        ParsedOutgoingNoteLink::parse_input_string.map(ParsedElement::ParsedOutgoingNoteLink),
                        ParsedCitation::parse_input_string.map(ParsedElement::ParsedCitation),
                        ParsedTag::parse_input_string.map(ParsedElement::Tag),
                        any.map(|c: char| ParsedElement::Text(c.to_string()))
                    )).parse_next(x)
                                    },
               _ => |x: &mut &str| {
                   alt((
                        ParsedInspectionRequest::parse_input_string.map(ParsedElement::ParsedInspectionRequest),
                       any.map(|c: char| ParsedElement::Text(c.to_string()))
                   )).parse_next(x)
               },
           }).parse(input)
             .map_err(|e| {
                 println!("Parsing Error: {:#?}", e);
                 FlusterError::ConundrumParsingError
             })
}
