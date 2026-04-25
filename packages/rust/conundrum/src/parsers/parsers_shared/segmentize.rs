use std::{cell::RefCell, rc::Rc};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    },
    parsers::{
        conundrum::{
            comment::ConundrumCommentResult, hr_with_children::hr_with_children_model::HrWithChildrenResult,
            inline_citation::ParsedCitation, note_link::note_link_model::ParsedOutgoingNoteLink, tag::ParsedTag,
        },
        markdown::{
            block_math::BlockMathResult, block_quote::block_quote_model::BlockQuoteResult,
            bold_and_italic_text::MarkdownBoldAndItalicTextResult, bold_text::MarkdownBoldTextResult,
            code_block::code_block_model::ParsedCodeBlock, heading::heading_model::MarkdownHeadingResult,
            hr::MarkdownHorizontalRule, inline_code::InlineCodeResult, inline_math::InlineMathResult,
            italic_text::MarkdownItalicTextResult, markdown_extensions::emoji::emoji_model::EmojiResult,
            markdown_link::MarkdownLinkResult, paragraph::paragraph_model::MarkdownParagraphResult,
        },
        parser_trait::ConundrumParser,
        react::{
            react_component_self_closing::ReactComponentSelfClosingResult,
            react_component_with_children::ReactComponentWithChildrenResult,
        },
    },
};

use winnow::token::{any, take};
use winnow::{Parser, combinator::alt};
use winnow::{
    combinator::{dispatch, fail, peek, repeat_till},
    token::literal,
};

pub fn until_paragraph_breaking_element<'a>(input: &mut ConundrumInput<'a>)
                                            -> ConundrumModalResult<Vec<ParsedElement>> {
    let at_line_start = Rc::new(RefCell::new(false));
    let at_line_start_terminator = Rc::clone(&at_line_start);
    let (res, terminator): (Vec<ParsedElement>, ParsedElement) =
                                                               repeat_till(
                                                                   1..,
                                                                   |nested_input: &mut ConundrumInput<'a>| {
                                                                       let inner_res = dispatch! {peek(take(1usize));
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
                                                                       }.parse_next(nested_input)?;

                                                                       let mut ls = at_line_start.borrow_mut();

                                                                       *ls = match &inner_res {
                                                                           ParsedElement::Text(t) => t == "\n",
                                                                           _ => false
                                                                       };

                                                                       Ok(inner_res)
                                                                   },
                                                                   peek(|nested_input: &mut ConundrumInput<'a>| {
                                                                       let ls = *at_line_start_terminator.borrow();
                                                                       let result =
                                                                           dispatch! {peek(take(1usize));
                                                                               "-" => |x: &mut ConundrumInput<'a>| {
                                                                                   if ls {
                                                                                       alt((
                                                                                               HrWithChildrenResult::parse_input_string.map(ParsedElement::HrWithChildren),
                                                                                               MarkdownHorizontalRule::parse_input_string.map(ParsedElement::Hr),
                                                                                       )).parse_next(x)
                                                                                   } else {
                                                                                       fail.parse_next(x)
                                                                                   }
                                                                               },
                                                                               "/" => |x: &mut ConundrumInput<'a>| {
                                                                                   if ls {
                                                                                       ConundrumCommentResult::parse_input_string.map(ParsedElement::Comment) .parse_next(x)
                                                                                   } else {
                                                                                       fail.parse_next(x)
                                                                                   }
                                                                               },
                                                                               "#" => |x: &mut ConundrumInput<'a>| {
                                                                                   if ls {
                                                                                       MarkdownHeadingResult::parse_input_string.map(ParsedElement::Heading) .parse_next(x)
                                                                                   } else {
                                                                                       fail.parse_next(x)
                                                                                   }
                                                                               },
                                                                               ">" => |x: &mut ConundrumInput<'a>| {
                                                                                   if ls {
                                                                                       BlockQuoteResult::parse_input_string.map(ParsedElement::BlockQuote) .parse_next(x)
                                                                                   } else {
                                                                                       fail.parse_next(x)
                                                                                   }
                                                                               },
                                                                               "`" => |x: &mut ConundrumInput<'a>| {
                                                                                   if ls {
                                                                                       ParsedCodeBlock::parse_input_string.map(ParsedElement::ParsedCodeBlock)
                                                                                           .parse_next(x)
                                                                                   } else {
                                                                                       fail.parse_next(x)
                                                                                   }
                                                                               },
                                                                               "$" => |x: &mut ConundrumInput<'a>| {
                                                                                   BlockMathResult::parse_input_string.map(ParsedElement::BlockMath) .parse_next(x)
                                                                               },
                                                                               "<" => |x: &mut ConundrumInput<'a>| {
                                                                                   alt((
                                                                                           ReactComponentWithChildrenResult::parse_input_string.verify(|comp| {
                                                                                               comp.component.component_is_block_level()
                                                                                           }).map(ParsedElement::ReactComponentWithChildren),
                                                                                           ReactComponentSelfClosingResult::parse_input_string.verify(|comp| {
                                                                                               comp.component.component_is_block_level()
                                                                                           }).map(ParsedElement::ReactComponentSelfClosing),
                                                                                   )).parse_next(x)
                                                                               },
                                                                               "\\" => |x: &mut ConundrumInput<'a>| {
                                                                                   (take(1usize).void(),take(1usize)).map(|(_, c): (_, &str)| {
                                                                                       ParsedElement::Text(c.to_string())
                                                                                   }).parse_next(x)
                                                                               },
                                                                               " " => |x: &mut ConundrumInput<'a>| {
                                                                                   literal("  \n").map(|_| {
                                                                                       ParsedElement::Text(String::from(""))
                                                                                   }).parse_next(x)
                                                                               },
                                                                               "\n" => |x: &mut ConundrumInput<'a>| {
                                                                                   literal("\n\n").map(|_| {
                                                                                       ParsedElement::Text(String::from(""))
                                                                                   }).parse_next(x)
                                                                               },
                                                                               _ => |x: &mut ConundrumInput<'a>| {
                                                                                   fail.parse_next(x)
                                                                               },
                                                                           }.parse_next(nested_input)?;
                                                                       Ok(result)
                                                                   }),
                                                                   ).parse_next(input)?;
    println!("Terminator: {:#?}", terminator);
    Ok(res)
}
