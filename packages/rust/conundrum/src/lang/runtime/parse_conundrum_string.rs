use winnow::combinator::{dispatch, peek};
use winnow::token::take;
use winnow::{
    Parser,
    combinator::{alt, repeat},
    token::any,
};

use crate::lang::runtime::apply_parsed_conundrum_result::apply_parsed_conundrum_input_state;
use crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult;
use crate::lang::runtime::traits::conundrum_input::ConundrumInput;
use crate::parsers::conundrum::comment::ConundrumCommentResult;
use crate::parsers::conundrum::docs::ParsedInspectionRequest;
use crate::parsers::conundrum::hr_with_children::HrWithChildrenResult;
use crate::parsers::markdown::block_math::BlockMathResult;
use crate::parsers::markdown::block_quote::BlockQuoteResult;
use crate::parsers::markdown::bold_and_italic_text::MarkdownBoldAndItalicTextResult;
use crate::parsers::markdown::bold_text::MarkdownBoldTextResult;
use crate::parsers::markdown::code_block::code_block_model::ParsedCodeBlock;
use crate::parsers::markdown::heading::MarkdownHeadingResult;
use crate::parsers::markdown::inline_code::InlineCodeResult;
use crate::parsers::markdown::inline_math::InlineMathResult;
use crate::parsers::markdown::italic_text::MarkdownItalicTextResult;
use crate::parsers::markdown::markdown_extensions::emoji::EmojiResult;
use crate::parsers::markdown::markdown_link::MarkdownLinkResult;
use crate::parsers::parser_trait::ConundrumParser;
use crate::parsers::react::react_component_self_closing::ReactComponentSelfClosingResult;
use crate::parsers::react::react_component_with_children::ReactComponentWithChildrenResult;
use crate::{
    lang::elements::parsed_elements::ParsedElement,
    parsers::conundrum::{inline_citation::ParsedCitation, note_link::ParsedOutgoingNoteLink, tag::ParsedTag},
};

/// Core recursive parser.  Returns a `ModalResult` so it can be called from
/// inside other winnow parsers (e.g. `BlockQuoteResult::parse_input_string`)
/// without a type-mismatch.
pub fn parse_elements<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<Vec<ParsedElement>> {
    let mut at_line_start: bool = true;
    repeat(0.., |input_inner: &mut ConundrumInput<'a>| -> ConundrumModalResult<ParsedElement> {
        let result =
            dispatch! {peek(take(1usize));
                "-" => |x: &mut ConundrumInput<'a>| {
                    if at_line_start {
                        alt((
                            HrWithChildrenResult::parse_input_string.map(ParsedElement::HrWithChildren),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                    } else {
                        any.map(|c: char| ParsedElement::Text(c.to_string())).parse_next(x)
                    }
                },
                "/" => |x: &mut ConundrumInput<'a>| {
                    if at_line_start {
                        alt((
                            ConundrumCommentResult::parse_input_string.map(ParsedElement::Comment),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                    } else {
                        any.map(|c: char| ParsedElement::Text(c.to_string())).parse_next(x)
                    }
                },
                "#" => |x: &mut ConundrumInput<'a>| {
                    if at_line_start {
                        alt((
                            MarkdownHeadingResult::parse_input_string.map(ParsedElement::Heading),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                    } else {
                        any.map(|c: char| ParsedElement::Text(c.to_string())).parse_next(x)
                    }
                },
                ">" => |x: &mut ConundrumInput<'a>| {
                    if at_line_start {
                        alt((
                            BlockQuoteResult::parse_input_string.map(ParsedElement::BlockQuote),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                    } else {
                        any.map(|c: char| ParsedElement::Text(c.to_string())).parse_next(x)
                    }
                },
                "`" => |x: &mut ConundrumInput<'a>| {
                    if at_line_start {
                        alt((
                            ParsedCodeBlock::parse_input_string.map(ParsedElement::ParsedCodeBlock),
                            InlineCodeResult::parse_input_string.map(ParsedElement::InlineCode),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                    } else {
                        alt((
                            InlineCodeResult::parse_input_string.map(ParsedElement::InlineCode),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                    }
                },
                "$" => |x: &mut ConundrumInput<'a>| {
                            alt((
                                BlockMathResult::parse_input_string.map(ParsedElement::BlockMath),
                                InlineMathResult::parse_input_string.map(ParsedElement::InlineMath),
                                any.map(|c: char| ParsedElement::Text(c.to_string()))
                            )).parse_next(x)
                },
                ":" => |x: &mut ConundrumInput<'a>| {
                            alt((
                                EmojiResult::parse_input_string.map(ParsedElement::Emoji),
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
                "<" => |x: &mut ConundrumInput<'a>| {
                    alt((
                       ReactComponentWithChildrenResult::parse_input_string.map(ParsedElement::ReactComponentWithChildren),
                       ReactComponentSelfClosingResult::parse_input_string.map(ParsedElement::ReactComponentSelfClosing),
                        any.map(|c: char| ParsedElement::Text(c.to_string()))
                    )).parse_next(x)
                },
                "\\" => |x: &mut ConundrumInput<'a>| {
                    (take(1usize).void(),take(1usize)).map(|(_, c): (_, &str)| {
                        ParsedElement::Text(c.to_string())
                    }).parse_next(x)
                },
                _ => |x: &mut ConundrumInput<'a>| {
                    if at_line_start {
                        alt((
                            ParsedInspectionRequest::parse_input_string.map(ParsedElement::ParsedInspectionRequest),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                    } else {
                        any.map(|c: char| ParsedElement::Text(c.to_string())).parse_next(x)
                    }
                },
            }.parse_next(input_inner)?;

        at_line_start = match &result {
            ParsedElement::Text(s) => s == "\n" || s == "\r\n",
            ParsedElement::ReactComponentWithChildren(c) => {
                c.component.component_is_block_level()
            },
            ParsedElement::ReactComponentSelfClosing(c) => {
                c.component.component_is_block_level()
            },
            ParsedElement::Heading(_)
            | ParsedElement::BlockQuote(_)
            | ParsedElement::BlockMath(_)
            | ParsedElement::MarkdownParagraph(_)
            | ParsedElement::ParsedInspectionRequest(_)
            | ParsedElement::HrWithChildren(_)
            | ParsedElement::Comment(_)
            | ParsedElement::ParsedCodeBlock(_) => true,
            _ => false,
        };

        Ok(result)
    }).parse_next(input)
}

/// Application-level entry point.  Parses the entire input and converts any
/// winnow error into a `FlusterError` for the rest of the app.
pub fn parse_conundrum_string<'a>(input: &'a mut ConundrumInput<'a>)
                                  -> ConundrumModalResult<(Vec<ParsedElement>, &'a mut ConundrumInput<'a>)> {
    let elements = parse_elements(input)?;
    apply_parsed_conundrum_input_state(input);
    Ok((elements, input))
}
