use serde::Serialize;
use winnow::{
    Parser,
    combinator::{alt, repeat, repeat_till, separated},
    error::ErrMode,
    token::take,
    token::{any, literal, take_until},
};

use crate::{
    lang::{
        lib::ui::ui_types::children::Children,
        runtime::{
            parse_conundrum_string::parse_elements,
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::ParseState,
            },
            traits::{
                conundrum_input::{ConundrumInput, get_conundrum_input},
                html_js_component_result::HtmlJsComponentResult,
                mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::auto_inserted_component_name::AutoInsertedComponentName,
    parsers::parser_trait::ConundrumParser,
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct MarkdownParagraphResult {
    pub children: Children,
}

impl HtmlJsComponentResult for MarkdownParagraphResult {
    fn to_html_js_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if self.children.0.is_empty() {
            Ok(String::from(""))
        } else {
            Ok(format!("<p>{}</p>", self.children.render(res)?))
        }
    }
}

impl PlainTextComponentResult for MarkdownParagraphResult {
    fn to_plain_text(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl MdxComponentResult for MarkdownParagraphResult {
    fn to_mdx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        let children_string = self.children.render(res)?;
        Ok(format!("<{}>\n{}\n</{}>",
                   AutoInsertedComponentName::AutoInsertedMarkdownParagraph,
                   children_string.trim(),
                   AutoInsertedComponentName::AutoInsertedMarkdownParagraph,))
    }
}

pub fn markdown_paragraph_line_break(input: &mut ConundrumInput) -> ConundrumModalResult<String> {
    alt((literal("  \n"), literal("\n\n"))).parse_next(input).map(String::from).map_err(|_| {
                                                                                   ErrMode::Backtrack(
            ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Fail to find paragraph break."))
        )
                                                                               })
}

pub fn markdown_paragraph(input: &mut ConundrumInput) -> ConundrumModalResult<String> {
    let (res, _): (Vec<&str>, String) =
        repeat_till(1.., take(1usize), markdown_paragraph_line_break).parse_next(input)?;
    let joined_paragraph = String::from_iter(res);
    Ok(joined_paragraph)
}

impl ConundrumParser<MarkdownParagraphResult> for MarkdownParagraphResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownParagraphResult> {
        // let res = alt((take_until(1.., "  \n"), take_until(1..,
        // "\n\n"))).parse_next(input)?;
        let res: Vec<String> = repeat(0.., markdown_paragraph_line_break).parse_next(input)?;
        let joined_paragraph = String::from_iter(res);
        let state = input.state.borrow_mut();
        let mut new_input = get_conundrum_input(&joined_paragraph, state.clone());
        let children = parse_elements(&mut new_input)?;
        // apply_nested_parser_state(input, &new_input);
        Ok(MarkdownParagraphResult { children: Children(children) })
    }

    fn matches_first_char(char: char) -> bool {
        char == '$'
    }
}
