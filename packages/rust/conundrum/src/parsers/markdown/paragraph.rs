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
        elements::parsed_elements::ParsedElement,
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
    parsers::{parser_trait::ConundrumParser, parsers_shared::segmentize::any_block_element},
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct MarkdownParagraphResult {
    pub children: Children,
    pub terminator: Option<Box<ParsedElement>>,
}

impl HtmlJsComponentResult for MarkdownParagraphResult {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        if self.children.0.is_empty() {
            Ok(String::from(""))
        } else {
            Ok(format!("<p>{}</p>{}", self.children.render(Arc::clone(&res))?, match &self.terminator {
                Some(s) => s.to_html_js_component(res)?,
                None => "".to_string(),
            }))
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

enum ParagraphTerminator {
    LineEnding,
    Content(ParsedElement),
}

pub fn markdown_paragraph(input: &mut ConundrumInput) -> ConundrumModalResult<(Children, Option<ParsedElement>)> {
    let (joined_paragraph, terminator) =
        repeat_till(1..,
                    take(1usize),
                    alt((markdown_paragraph_line_break.map(|_| {
                        ParagraphTerminator::LineEnding
                    }),
                        any_block_element.map(ParagraphTerminator::Content)))).verify_map(|(res, terminator): (Vec<&str>, ParagraphTerminator)| {
                                                                          let joined_paragraph = String::from_iter(res);
                                                                          println!("Joined Paragraph: {:#?}",
                                                                                   joined_paragraph);
                                                                          if joined_paragraph.trim().is_empty() {
                                                                              None
                                                                          } else {
                                                                          let terminator_data = match terminator {
                                                                              ParagraphTerminator::Content(c) => Some(c),
                                                                              ParagraphTerminator::LineEnding => None
                                                                          };
                                                                              Some((joined_paragraph, terminator_data))
                                                                          }
                                                                      })
                                                                      .parse_next(input)?;

    let mut new_input = ConundrumInput { input: &joined_paragraph,
                                         state: Arc::clone(&input.state) };
    let children = parse_elements(&mut new_input)?;
    Ok((Children(children), terminator))
}

impl ConundrumParser<MarkdownParagraphResult> for MarkdownParagraphResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownParagraphResult> {
        // let res = alt((take_until(1.., "  \n"), take_until(1..,
        // "\n\n"))).parse_next(input)?;
        let (children, terminator) = markdown_paragraph.parse_next(input)?;
        Ok(MarkdownParagraphResult { children,
                                     terminator: terminator.map(Box::new) })
    }

    fn matches_first_char(char: char) -> bool {
        char == '$'
    }
}
