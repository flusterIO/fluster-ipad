use std::io::Error;

use fluster_core_utilities::core_types::fluster_error::{FlusterError, FlusterResult};
use winnow::combinator::trace;
use winnow::combinator::{dispatch, opt, peek};
use winnow::stream::Stream;
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
                   trace(
                       "Some Parser",
                       alt((
                        ParsedOutgoingNoteLink::parse_input_string.map(ParsedElement::OutgoingNoteLink),
                       ParsedCitation::parse_input_string.map(ParsedElement::Citation),
                       ParsedTag::parse_input_string.map(ParsedElement::Tag),
                       any.map(|c: char| ParsedElement::Text(c.to_string()))
                       ))
                   ).parse_next(x)
                   // if let Ok(res) = opt(alt((
                   //     ParsedOutgoingNoteLink::parse_input_string.map(ParsedElement::OutgoingNoteLink),
                   //     ParsedCitation::parse_input_string.map(ParsedElement::Citation),
                   //     ParsedTag::parse_input_string.map(ParsedElement::Tag)
                   // )))
                   //     .parse_next("")
                   //   {
                   //  match res {
                   //     Some(element) => Ok(element),
                   //     None => {
                   //         any.map(|c: char| ParsedElement::Text(c.to_string())).parse_next(x)
                   //     }
                   // }
                   // } else {
                   //         any.map(|c: char| ParsedElement::Text(c.to_string())).parse_next(x)
                   // }
               },

               _ => |x: &mut &str| {
                   opt(ParsedInspectionRequest::parse_input_string.map(ParsedElement::InspectionRequest)).parse_next(x).map_or_else(|z| {
                        Err(z)
                   }, |y| {
                        y.map(|w| {
                        Ok(w)
                        }).unwrap_or_else(|| {
                           any.map(|c: char| ParsedElement::Text(c.to_string())).parse_next(x)
                        })
                   })
               },
           }).parse(input).map_err(|e| {
               println!("Parsing Error: {:#?}", e);
               FlusterError::ConundrumParsingError
           })
}
