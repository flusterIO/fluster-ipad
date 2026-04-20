use serde::Serialize;
use std::sync::Arc;
use winnow::{
    Parser,
    combinator::{alt, repeat_till},
    error::{ContextError, ErrMode},
    token::{literal, take},
};

use crate::{
    lang::{
        lib::ui::ui_types::children::Children,
        runtime::{
            parse_conundrum_string::parse_elements,
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            },
            traits::{
                conundrum_input::{ArcState, ConundrumInput},
                html_js_component_result::HtmlJsComponentResult,
                mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::auto_inserted_component_name::AutoInsertedComponentName,
    parsers::{
        markdown::block_quote::{
            self,
            block_quote_model::{self, BlockQuoteResult},
        },
        parser_trait::ConundrumParser,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct MarkdownParagraphResult {
    pub children: Children,
}

impl HtmlJsComponentResult for MarkdownParagraphResult {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        if self.children.0.is_empty() {
            Ok(String::from(""))
        } else {
            Ok(format!("<p>{}</p>", self.children.render(res)?))
        }
    }
}

impl PlainTextComponentResult for MarkdownParagraphResult {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl MdxComponentResult for MarkdownParagraphResult {
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let children_string = self.children.render(res)?;
        Ok(format!("<{}>\n{}\n</{}>",
                   AutoInsertedComponentName::AutoInsertedMarkdownParagraph,
                   children_string.trim(),
                   AutoInsertedComponentName::AutoInsertedMarkdownParagraph,))
    }
}

pub fn markdown_paragraph_line_break(input: &mut ConundrumInput) -> ConundrumModalResult<String> {
    alt((literal("  \n"), literal("\n\n"))).parse_next(input).map(String::from).map_err(|_: ContextError| {
                                                                                   ErrMode::Backtrack(
            ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Fail to find paragraph break."))
        )
                                                                               })
}

pub fn markdown_paragraph(input: &mut ConundrumInput) -> ConundrumModalResult<String> {
    let res =
        repeat_till(1.., take(1usize), markdown_paragraph_line_break).verify_map(|(res, _): (Vec<&str>, String)| {
                                                                         let joined_paragraph = String::from_iter(res);
                                                                         println!("Joined Paragraph: {:#?}",
                                                                                  joined_paragraph);
                                                                         if joined_paragraph.trim().is_empty() {
                                                                             None
                                                                         } else {
                                                                             Some(joined_paragraph)
                                                                         }
                                                                     })
                                                                     .parse_next(input)?;
    Ok(res)
}

impl ConundrumParser<MarkdownParagraphResult> for MarkdownParagraphResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownParagraphResult> {
        // let res = alt((take_until(1.., "  \n"), take_until(1..,
        // "\n\n"))).parse_next(input)?;
        let joined_paragraph = markdown_paragraph.parse_next(input)?;
        let mut new_input = ConundrumInput { input: &joined_paragraph,
                                             state: Arc::clone(&input.state) };
        let children = parse_elements(&mut new_input)?;
        Ok(MarkdownParagraphResult { children: Children(children) })
    }

    fn matches_first_char(char: char) -> bool {
        char == '$'
    }
}
