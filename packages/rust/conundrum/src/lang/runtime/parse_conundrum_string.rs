use fluster_core_utilities::core_types::fluster_error::{FlusterError, FlusterResult};
use winnow::combinator::{dispatch, peek};
use winnow::token::take;
use winnow::{
    ModalResult, Parser,
    combinator::{alt, repeat},
    token::any,
};

use crate::lang::runtime::traits::conundrum_input::ConundrumInput;
use crate::parsers::fluster::docs::ParsedInspectionRequest;
use crate::parsers::markdown::block_math::BlockMathResult;
use crate::parsers::markdown::block_quote::BlockQuoteResult;
use crate::parsers::markdown::bold_and_italic_text::MarkdownBoldAndItalicTextResult;
use crate::parsers::markdown::bold_text::MarkdownBoldTextResult;
use crate::parsers::markdown::code_block::ParsedCodeBlock;
use crate::parsers::markdown::heading::MarkdownHeadingResult;
use crate::parsers::markdown::inline_math::InlineMathResult;
use crate::parsers::markdown::italic_text::MarkdownItalicTextResult;
use crate::parsers::markdown::markdown_link::MarkdownLinkResult;
use crate::parsers::parser_trait::ConundrumParser;
use crate::{
    lang::elements::parsed_elements::ParsedElement,
    parsers::fluster::{inline_citation::ParsedCitation, note_link::ParsedOutgoingNoteLink, tag::ParsedTag},
};

/// Core recursive parser.  Returns a `ModalResult` so it can be called from
/// inside other winnow parsers (e.g. `BlockQuoteResult::parse_input_string`)
/// without a type-mismatch.
pub fn parse_elements<'a>(input_root: &mut ConundrumInput<'a>) -> ModalResult<Vec<ParsedElement>> {
    // Documents and recursively-parsed inner blocks always begin at a line
    // boundary.
    let mut at_line_start = true;

    repeat(0.., |input: &mut ConundrumInput<'a>| -> ModalResult<ParsedElement> {
        let result =
            dispatch! {peek(take(1usize));
                "\n" => |x: &mut ConundrumInput<'a>| {
                        alt((
                            MarkdownHeadingResult::parse_input_string.map(ParsedElement::Heading),
                            BlockQuoteResult::parse_input_string.map(ParsedElement::BlockQuote),
                            ParsedInspectionRequest::parse_input_string.map(ParsedElement::ParsedInspectionRequest),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                },
                "#" => |x: &mut ConundrumInput<'a>| {
                        alt((
                            MarkdownHeadingResult::parse_input_string.map(ParsedElement::Heading),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                },
                ">" => |x: &mut ConundrumInput<'a>| {
                        alt((
                            BlockQuoteResult::parse_input_string.map(ParsedElement::BlockQuote),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                },
                "`" => |x: &mut ConundrumInput<'a>| {
                    alt((
                        ParsedCodeBlock::parse_input_string.map(ParsedElement::ParsedCodeBlock),
                        any.map(|c: char| ParsedElement::Text(c.to_string()))
                    )).parse_next(x)
                },
                "$" => |x: &mut ConundrumInput<'a>| {
                    alt((
                        BlockMathResult::parse_input_string.map(ParsedElement::BlockMath),
                        InlineMathResult::parse_input_string.map(ParsedElement::InlineMath),
                        any.map(|c: char| ParsedElement::Text(c.to_string()))
                    )).parse_next(x)
                },
                "[" => |x: &mut ConundrumInput<'a>| {
                    alt((
                        ParsedOutgoingNoteLink::parse_input_string.map(ParsedElement::ParsedOutgoingNoteLink),
                        ParsedCitation::parse_input_string.map(ParsedElement::ParsedCitation),
                        ParsedTag::parse_input_string.map(ParsedElement::Tag),
                        MarkdownLinkResult::parse_input_string.map(ParsedElement::MarkdownLink),
                        any.map(|c: char| ParsedElement::Text(c.to_string()))
                    )).parse_next(x)
                },
                "*" | "_" => |x: &mut ConundrumInput<'a>| {
                    alt((
                       MarkdownBoldAndItalicTextResult::parse_input_string.map(ParsedElement::BoldAndItalicText),
                       MarkdownBoldTextResult::parse_input_string.map(ParsedElement::BoldText),
                       MarkdownItalicTextResult::parse_input_string.map(ParsedElement::ItalicText),
                        any.map(|c: char| ParsedElement::Text(c.to_string()))
                    )).parse_next(x)
                },
                _ => |x: &mut ConundrumInput<'a>| {
                    alt((
                        ParsedInspectionRequest::parse_input_string.map(ParsedElement::ParsedInspectionRequest),
                        any.map(|c: char| ParsedElement::Text(c.to_string()))
                    )).parse_next(x)
                },
            }.parse_next(input)?;

        // A Text element from `any` is a line start only when it was a newline.
        // Block-level elements consume their own trailing newline, so the
        // position after them is always a line start.
        at_line_start = match &result {
            ParsedElement::Text(s) => s == "\n" || s == "\r\n",
            ParsedElement::Heading(_)
            | ParsedElement::BlockQuote(_)
            | ParsedElement::BlockMath(_)
            | ParsedElement::ParsedCodeBlock(_) => true,
            _ => false,
        };

        Ok(result)
    }).parse_next(input_root)
}

/// Application-level entry point.  Parses the entire input and converts any
/// winnow error into a `FlusterError` for the rest of the app.
pub fn parse_conundrum_string<'a>(input: &mut ConundrumInput<'a>) -> FlusterResult<Vec<ParsedElement>> {
    parse_elements(input).map_err(|e| {
                             println!("Parsing Error: {:#?}", e);
                             FlusterError::ConundrumParsingError
                         })
}
