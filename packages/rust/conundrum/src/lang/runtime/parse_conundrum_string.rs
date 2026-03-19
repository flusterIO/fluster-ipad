use fluster_core_utilities::core_types::fluster_error::{FlusterError, FlusterResult};
use winnow::combinator::{dispatch, peek};
use winnow::token::take;
use winnow::{
    ModalResult, Parser,
    combinator::{alt, repeat},
    token::any,
};

use crate::parsers::fluster::docs::ParsedInspectionRequest;
use crate::parsers::markdown::block_math::BlockMathResult;
use crate::parsers::markdown::code_block::ParsedCodeBlock;
use crate::parsers::markdown::heading::MarkdownHeadingResult;
use crate::parsers::markdown::inline_math::InlineMathResult;
use crate::parsers::parser_trait::ConundrumParser;
use crate::{
    lang::elements::parsed_elements::ParsedElement,
    parsers::fluster::{inline_citation::ParsedCitation, note_link::ParsedOutgoingNoteLink, tag::ParsedTag},
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
               "#" => |x: &mut &str| {
                   alt((
                       MarkdownHeadingResult::parse_input_string.map(ParsedElement::Heading),
                       any.map(|c: char| ParsedElement::Text(c.to_string()))
                   )).parse_next(x)
               },
               "$" => |x: &mut &str | {
                   alt((
                        BlockMathResult::parse_input_string.map(ParsedElement::BlockMath),
                        InlineMathResult::parse_input_string.map(ParsedElement::InlineMath),
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
