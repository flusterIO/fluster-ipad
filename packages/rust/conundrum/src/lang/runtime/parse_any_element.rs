use nom::{IResult, Parser, branch::alt};

use crate::{
    lang::elements::parsed_elements::ParsedElement,
    parsers::{
        fluster::{docs::parse_inspection, inline_citation::parse_citation},
        markdown::code_block::parse_code_block,
    },
};

pub fn parse_any_element(input: &str) -> IResult<&str, ParsedElement> {
    // alt tries each parser in order. The .map() method (which requires the Parser trait)
    // wraps the result in our ParsedElement enum so both return the same type.
    alt((
        parse_code_block.map(ParsedElement::CodeBlock),
        parse_citation.map(ParsedElement::Citation),
        parse_inspection.map(ParsedElement::InspectionRequest),
    ))
    .parse(input)
}
