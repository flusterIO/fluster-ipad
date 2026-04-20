use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    },
    parsers::{
        conundrum::{
            comment::ConundrumCommentResult, docs::ParsedInspectionRequest, hr_with_children::HrWithChildrenResult,
            inline_citation::ParsedCitation, note_link::note_link_model::ParsedOutgoingNoteLink, tag::ParsedTag,
        },
        markdown::{
            block_math::BlockMathResult, block_quote::block_quote_model::BlockQuoteResult,
            bold_and_italic_text::MarkdownBoldAndItalicTextResult, bold_text::MarkdownBoldTextResult,
            code_block::code_block_model::ParsedCodeBlock, heading::heading_model::MarkdownHeadingResult,
            hr::MarkdownHorizontalRule, inline_code::InlineCodeResult, inline_math::InlineMathResult,
            italic_text::MarkdownItalicTextResult, markdown_extensions::emoji::EmojiResult,
            markdown_link::MarkdownLinkResult, paragraph::MarkdownParagraphResult,
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

pub fn segmentize<'a>(input: &'a mut ConundrumInput<'a>) -> ConundrumModalResult<Vec<ParsedElement>> {
    let r: Vec<ParsedElement> = repeat(0.., |input_inner: &mut ConundrumInput<'a>| -> ConundrumModalResult<ParsedElement> {
        let result =
            dispatch! {peek(take(1usize));
                "-" => |x: &mut ConundrumInput<'a>| {
                        alt((
                            HrWithChildrenResult::parse_input_string.map(ParsedElement::HrWithChildren),
                            MarkdownHorizontalRule::parse_input_string.map(ParsedElement::Hr),
                            MarkdownParagraphResult::parse_input_string.map(ParsedElement::MarkdownParagraph),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                },
                "/" => |x: &mut ConundrumInput<'a>| {
                        alt((
                            ConundrumCommentResult::parse_input_string.map(ParsedElement::Comment),
                            MarkdownParagraphResult::parse_input_string.map(ParsedElement::MarkdownParagraph),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                },
                "#" => |x: &mut ConundrumInput<'a>| {
                        alt((
                            MarkdownHeadingResult::parse_input_string.map(ParsedElement::Heading),
                            MarkdownParagraphResult::parse_input_string.map(ParsedElement::MarkdownParagraph),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                },
                ">" => |x: &mut ConundrumInput<'a>| {
                        alt((
                            BlockQuoteResult::parse_input_string.map(ParsedElement::BlockQuote),
                            MarkdownParagraphResult::parse_input_string.map(ParsedElement::MarkdownParagraph),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                },
                "`" => |x: &mut ConundrumInput<'a>| {
                        alt((
                            ParsedCodeBlock::parse_input_string.map(ParsedElement::ParsedCodeBlock),
                            InlineCodeResult::parse_input_string.map(ParsedElement::InlineCode),
                            MarkdownParagraphResult::parse_input_string.map(ParsedElement::MarkdownParagraph),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                },
                "$" => |x: &mut ConundrumInput<'a>| {
                            alt((
                                BlockMathResult::parse_input_string.map(ParsedElement::BlockMath),
                                InlineMathResult::parse_input_string.map(ParsedElement::InlineMath),
                            MarkdownParagraphResult::parse_input_string.map(ParsedElement::MarkdownParagraph),
                                any.map(|c: char| ParsedElement::Text(c.to_string()))
                            )).parse_next(x)
                },
                ":" => |x: &mut ConundrumInput<'a>| {
                            alt((
                                EmojiResult::parse_input_string.map(ParsedElement::Emoji),
                            MarkdownParagraphResult::parse_input_string.map(ParsedElement::MarkdownParagraph),
                                any.map(|c: char| ParsedElement::Text(c.to_string()))
                            )).parse_next(x)
                },
                "[" => |x: &mut ConundrumInput<'a>| {
                    alt((
                        ParsedOutgoingNoteLink::parse_input_string.map(ParsedElement::ParsedOutgoingNoteLink),
                        ParsedCitation::parse_input_string.map(ParsedElement::ParsedCitation),
                        ParsedTag::parse_input_string.map(ParsedElement::Tag),
                        MarkdownLinkResult::parse_input_string.map(ParsedElement::MarkdownLink),
                            MarkdownParagraphResult::parse_input_string.map(ParsedElement::MarkdownParagraph),
                        any.map(|c: char| ParsedElement::Text(c.to_string()))
                    )).parse_next(x)
                },
                "*" | "_" => |x: &mut ConundrumInput<'a>| {
                    alt((
                       MarkdownBoldAndItalicTextResult::parse_input_string.map(ParsedElement::BoldAndItalicText),
                        MarkdownHorizontalRule::parse_input_string.map(ParsedElement::Hr),
                       MarkdownBoldTextResult::parse_input_string.map(ParsedElement::BoldText),
                       MarkdownItalicTextResult::parse_input_string.map(ParsedElement::ItalicText),
                            MarkdownParagraphResult::parse_input_string.map(ParsedElement::MarkdownParagraph),
                        any.map(|c: char| ParsedElement::Text(c.to_string()))
                    )).parse_next(x)
                },
                "<" => |x: &mut ConundrumInput<'a>| {
                    alt((
                       ReactComponentWithChildrenResult::parse_input_string.map(ParsedElement::ReactComponentWithChildren),
                       ReactComponentSelfClosingResult::parse_input_string.map(ParsedElement::ReactComponentSelfClosing),
                            MarkdownParagraphResult::parse_input_string.map(ParsedElement::MarkdownParagraph),
                        any.map(|c: char| ParsedElement::Text(c.to_string()))
                    )).parse_next(x)
                },
                "\\" => |x: &mut ConundrumInput<'a>| {
                    (take(1usize).void(),take(1usize)).map(|(_, c): (_, &str)| {
                        ParsedElement::Text(c.to_string())
                    }).parse_next(x)
                },
                _ => |x: &mut ConundrumInput<'a>| {
                        alt((
                            MarkdownParagraphResult::parse_input_string.map(ParsedElement::MarkdownParagraph),
                            ParsedInspectionRequest::parse_input_string.map(ParsedElement::ParsedInspectionRequest),
                            any.map(|c: char| ParsedElement::Text(c.to_string()))
                        )).parse_next(x)
                },
            }.parse_next(input_inner)?;
        Ok(result)
    }).parse_next(input)?;
    Ok(r)
}
