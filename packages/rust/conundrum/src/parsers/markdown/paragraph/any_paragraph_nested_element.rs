use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    },
    parsers::{
        conundrum::{
            inline_citation::ParsedCitation, note_link::note_link_model::ParsedOutgoingNoteLink, tag::ParsedTag,
        },
        markdown::{
            bold_and_italic_text::MarkdownBoldAndItalicTextResult, bold_text::MarkdownBoldTextResult,
            hr::MarkdownHorizontalRule, inline_code::InlineCodeResult, inline_math::InlineMathResult,
            italic_text::MarkdownItalicTextResult, markdown_extensions::emoji::emoji_model::EmojiResult,
            markdown_link::MarkdownLinkResult,
        },
        parser_trait::ConundrumParser,
        react::{
            react_component_self_closing::ReactComponentSelfClosingResult,
            react_component_with_children::ReactComponentWithChildrenResult,
        },
    },
};
use winnow::combinator::{dispatch, peek};
use winnow::token::take;
use winnow::{
    Parser,
    combinator::{alt, repeat},
    token::any,
};

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
                    any.map(|c: char| ParsedElement::Text(c.to_string()))
            )).parse_next(x)
        },
        "*" | "_" => |x: &mut ConundrumInput<'a>| {
            alt((
                    MarkdownBoldAndItalicTextResult::parse_input_string.map(ParsedElement::BoldAndItalicText),
                    MarkdownHorizontalRule::parse_input_string.map(ParsedElement::Hr),
                    MarkdownBoldTextResult::parse_input_string.map(ParsedElement::BoldText),
                    MarkdownItalicTextResult::parse_input_string.map(ParsedElement::ItalicText),
                    any.map(|c: char| ParsedElement::Text(c.to_string()))
            )).parse_next(x)
        },
        "<" => |x: &mut ConundrumInput<'a>| {
            alt((
                    ReactComponentWithChildrenResult::parse_input_string.verify(|item| {
                        !item.component.component_is_block_level()
                    }).map(ParsedElement::ReactComponentWithChildren),
                    ReactComponentSelfClosingResult::parse_input_string.verify(|item| {
                        !item.component.component_is_block_level()
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
