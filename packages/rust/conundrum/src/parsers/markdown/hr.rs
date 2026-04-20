use crate::lang::runtime::state::conundrum_error::ConundrumError;
use crate::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;
use crate::lang::runtime::traits::conundrum_input::ArcState;
use crate::lang::runtime::traits::fluster_component_result::ConundrumComponentResult;
use crate::lang::runtime::traits::html_js_component_result::HtmlJsComponentResult;
use crate::lang::runtime::traits::mdx_component_result::MdxComponentResult;
use crate::lang::runtime::traits::plain_text_component_result::PlainTextComponentResult;
use crate::lang::runtime::{
    state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput,
};
use crate::parsers::parser_trait::ConundrumParser;
use serde::Serialize;
use winnow::Parser;
use winnow::combinator::alt;
use winnow::error::{ContextError, ErrMode};
use winnow::token::literal;

pub fn horizontal_rule(input: &mut ConundrumInput) -> ConundrumModalResult<()> {
    alt((literal("---").void(), literal("***").void(), literal("___").void())).parse_next(input)
                                                                              .map_err(|_: ContextError| {
                                                                                  ErrMode::Backtrack(
            ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Not a valid horizontal rule."))
            )
                                                                              })?;
    Ok(())
}

#[derive(Serialize, Clone, Debug)]
pub struct MarkdownHorizontalRule {}

impl HtmlJsComponentResult for MarkdownHorizontalRule {
    fn to_html_js_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(String::from("<hr class=\"w-full h-2 bg-muted/80\" />"))
    }
}

impl PlainTextComponentResult for MarkdownHorizontalRule {
    fn to_plain_text(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(String::from("---"))
    }
}

impl MdxComponentResult for MarkdownHorizontalRule {
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.to_html_js_component(res)
    }
}

impl ConundrumComponentResult for MarkdownHorizontalRule {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.targets_html_js() {
            drop(state);
            self.to_html_js_component(res)
        } else if state.is_plain_text() {
            drop(state);
            self.to_plain_text(res)
        } else {
            drop(state);
            self.to_mdx_component(res)
        }
    }
}

impl ConundrumParser<MarkdownHorizontalRule> for MarkdownHorizontalRule {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownHorizontalRule> {
        horizontal_rule.parse_next(input)?;
        Ok(MarkdownHorizontalRule {})
    }

    fn matches_first_char(char: char) -> bool {
        char == '*' || char == '-' || char == '_'
    }
}
