use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    },
    parsers::{
        conundrum::{
            comment::ConundrumCommentResult, docs::ParsedInspectionRequest,
            hr_with_children::hr_with_children_model::HrWithChildrenResult,
        },
        markdown::{
            block_math::BlockMathResult, block_quote::block_quote_model::BlockQuoteResult,
            code_block::code_block_model::ParsedCodeBlock, heading::heading_model::MarkdownHeadingResult,
            hr::MarkdownHorizontalRule,
        },
        parser_trait::ConundrumParser,
        react::{
            react_component_self_closing::ReactComponentSelfClosingResult,
            react_component_with_children::ReactComponentWithChildrenResult,
        },
    },
};

use winnow::combinator::{dispatch, fail, peek};
use winnow::token::take;
use winnow::{Parser, combinator::alt};

/// This represent every element capapble of breaking out of the middle of a
/// paragraph.
pub fn any_block_element<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<ParsedElement> {
    let result = dispatch! {peek(take(1usize));
                     // "-" => |x: &mut ConundrumInput<'a>| {
                     //     alt((
                     //             HrWithChildrenResult::parse_input_string.map(ParsedElement::HrWithChildren),
                     //             MarkdownHorizontalRule::parse_input_string.map(ParsedElement::Hr),
                     //     )).parse_next(x)
                     // },
                     // "/" => |x: &mut ConundrumInput<'a>| {
                     //             ConundrumCommentResult::parse_input_string.map(ParsedElement::Comment).parse_next(x)
                     // },
                     // "#" => |x: &mut ConundrumInput<'a>| {
                     //             MarkdownHeadingResult::parse_input_string.map(ParsedElement::Heading).parse_next(x)
                     // },
                     "`" => |x: &mut ConundrumInput<'a>| {
                                 ParsedCodeBlock::parse_input_string.map(ParsedElement::ParsedCodeBlock).parse_next(x)
                     },
                     "$" => |x: &mut ConundrumInput<'a>| {
                                 BlockMathResult::parse_input_string.map(ParsedElement::BlockMath).parse_next(x)
                     },
                     // "*" | "_" => |x: &mut ConundrumInput<'a>| {
                     //             MarkdownHorizontalRule::parse_input_string.map(ParsedElement::Hr).parse_next(x)
                     // },
                     "<" => |x: &mut ConundrumInput<'a>| {
                         alt((
                                 ReactComponentWithChildrenResult::parse_input_string.verify(|c| {
                                     c.component.component_is_block_level()
                                 }).map(ParsedElement::ReactComponentWithChildren),
                                 ReactComponentSelfClosingResult::parse_input_string.verify(|c| {
                                     c.component.component_is_block_level()
                                 }).map(ParsedElement::ReactComponentSelfClosing),
                         )).parse_next(x)
                     },
                     _ => |x: &mut ConundrumInput<'a>| {
                                 fail.parse_next(x)
                     },
                 }.parse_next(input)?;
    println!("Result: {:#?}", result);
    Ok(result)
}
