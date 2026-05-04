use crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult;
use crate::lang::runtime::traits::conundrum_input::ConundrumInput;
use crate::parsers::conundrum::note_link::note_link_model::ParsedOutgoingNoteLink;
use crate::parsers::markdown::bold_and_italic_text::MarkdownBoldAndItalicTextResult;
use crate::parsers::markdown::bold_text::MarkdownBoldTextResult;
use crate::parsers::markdown::inline_code::InlineCodeResult;
use crate::parsers::markdown::italic_text::MarkdownItalicTextResult;
use crate::parsers::markdown::markdown_extensions::emoji::emoji_model::EmojiResult;
use crate::parsers::markdown::markdown_link::MarkdownLinkResult;
use crate::parsers::markdown::math::inline_math::inline_math_model::InlineMathResult;
use crate::parsers::parser_trait::ConundrumParser;
use crate::parsers::parsers_shared::escape_handling::escaped_char;
use crate::parsers::react::react_component_self_closing::ReactComponentSelfClosingResult;
use crate::parsers::react::react_component_with_children::ReactComponentWithChildrenResult;
use crate::{
    lang::elements::parsed_elements::ParsedElement,
    parsers::conundrum::{inline_citation::ParsedCitation, tag::ParsedTag},
};
use winnow::combinator::{dispatch, peek};
use winnow::token::take;
use winnow::{Parser, combinator::alt, token::any};

/// Accepts only elements that can go in places that are **completely** inline,
/// like a table cell or some title use cases.
pub fn parse_inline_element<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<ParsedElement> {
    dispatch! {peek(take(1usize));
                "`" => |x: &mut ConundrumInput<'a>| {
                        alt((
                            InlineCodeResult::parse_input_string.map(ParsedElement::InlineCode),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                },
                "$" => |x: &mut ConundrumInput<'a>| {
                            alt((
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
                       ReactComponentWithChildrenResult::parse_input_string.verify(|c| !c.component.component_is_block_level()).map(ParsedElement::ReactComponentWithChildren),
                       ReactComponentSelfClosingResult::parse_input_string.verify(|c| !c.component.component_is_block_level()).map(ParsedElement::ReactComponentSelfClosing),
                        any.map(|c: char| ParsedElement::Text(c.to_string()))
                    )).parse_next(x)
                },
                "\\" => |x: &mut ConundrumInput<'a>| {
                    escaped_char.map(|c| ParsedElement::Text(c.to_string())).parse_next(x)
                },
                _ => |x: &mut ConundrumInput<'a>| {
                        any.map(|c: char| ParsedElement::Text(c.to_string())).parse_next(x)
                },
            }.parse_next(input)
}
