use winnow::combinator::{dispatch, peek};
use winnow::token::{any, take};
use winnow::{Parser, combinator::alt};

use crate::lang::elements::parsed_elements::ParsedElement;
use crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult;
use crate::lang::runtime::traits::conundrum_input::ConundrumInput;
use crate::parsers::conundrum::inline_citation::ParsedCitation;
use crate::parsers::conundrum::note_link::note_link_model::ParsedOutgoingNoteLink;
use crate::parsers::conundrum::tag::ParsedTag;
use crate::parsers::markdown::bold_and_italic_text::MarkdownBoldAndItalicTextResult;
use crate::parsers::markdown::bold_text::MarkdownBoldTextResult;
use crate::parsers::markdown::inline_code::InlineCodeResult;
use crate::parsers::markdown::italic_text::MarkdownItalicTextResult;
use crate::parsers::markdown::markdown_extensions::emoji::emoji_model::EmojiResult;
use crate::parsers::markdown::markdown_extensions::footnote::footnote_anchor::FootnoteAnchor;
use crate::parsers::markdown::markdown_link::MarkdownLinkResult;
use crate::parsers::markdown::math::inline_math::inline_math_model::InlineMathResult;
use crate::parsers::parser_trait::ConundrumParser;
use crate::parsers::react::react_component_self_closing::ReactComponentSelfClosingResult;
use crate::parsers::react::react_component_with_children::ReactComponentWithChildrenResult;

/// The only job for this parser is to succeed in the context of
/// `repeat_till(0.., this one, some other one)` for all elements that can be
/// nested within a parargraph.
pub fn any_paragraph_nested_element<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<ParsedElement> {
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
                    FootnoteAnchor::parse_input_string.map(ParsedElement::FootnoteAnchor),
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
                    ReactComponentWithChildrenResult::parse_input_string.verify(|comp| {
                        !comp.component.component_is_block_level()
                    }).map(ParsedElement::ReactComponentWithChildren),
                    ReactComponentSelfClosingResult::parse_input_string.verify(|comp| {
                        !comp.component.component_is_block_level()
                    }).map(ParsedElement::ReactComponentSelfClosing),
                    any.map(|c: char| ParsedElement::Text(c.to_string()))
            )).parse_next(x)
        },
        "\\" => |x: &mut ConundrumInput<'a>| {
            (take(1usize).void(),take(1usize)).map(|(_, c): (_, &str)| {
                ParsedElement::Text(c.to_string())
            }).parse_next(x)
        },
        _ => |x: &mut ConundrumInput<'a>| {
                any.map(|c: char| ParsedElement::Text(c.to_string())).parse_next(x)
        },
    }.parse_next(input)
}
