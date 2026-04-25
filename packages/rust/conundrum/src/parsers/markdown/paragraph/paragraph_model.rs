use serde::Serialize;
use std::sync::Arc;
use winnow::{
    Parser,
    combinator::alt,
    error::{ContextError, ErrMode},
    stream::Stream,
    token::literal,
};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::ui_types::children::Children,
        runtime::{
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
    parsers::{parser_trait::ConundrumParser, parsers_shared::segmentize::until_paragraph_breaking_element},
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct MarkdownParagraphResult {
    pub children: Children,
    pub terminator: Box<ParsedElement>,
}

impl MarkdownParagraphResult {
    /// Returns true if the children should break out of the paragraph,
    /// rendering without the containing `<p>`.
    pub fn should_breakout(&self) -> bool {
        if self.children.0.is_empty() {
            false
        } else {
            !self.children.is_all_inline()
        }
    }
}

impl HtmlJsComponentResult for MarkdownParagraphResult {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let children = self.children.render(Arc::clone(&res))?;
        if children.trim().is_empty() {
            self.terminator.to_html_js_component(Arc::clone(&res))
        } else {
            Ok(format!("<p>{}</p>{}", children, self.terminator.to_html_js_component(Arc::clone(&res))?))
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

pub fn markdown_paragraph(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownParagraphResult> {
    let start = input.input.checkpoint();
    let (children, terminator) = until_paragraph_breaking_element.parse_next(input).inspect_err(|_| {
                                                                                        input.input.reset(&start);
                                                                                    })?;
    Ok(MarkdownParagraphResult { children: Children(children),
                                 terminator: Box::new(terminator) })
}

impl ConundrumParser<MarkdownParagraphResult> for MarkdownParagraphResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownParagraphResult> {
        markdown_paragraph.parse_next(input)
    }

    fn matches_first_char(char: char) -> bool {
        char != ' '
    }
}
