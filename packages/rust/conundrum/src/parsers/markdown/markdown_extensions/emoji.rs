use serde::Serialize;
use winnow::{Parser, combinator::delimited, stream::AsChar, token::take_while};

use crate::{
    lang::runtime::{
        state::{conundrum_error_variant::ConundrumModalResult, parse_state::ConundrumModifier},
        traits::{
            fluster_component_result::ConundrumComponentResult, jsx_component_result::JsxComponentResult,
            markdown_component_result::MarkdownComponentResult, mdx_component_result::MdxComponentResult,
            plain_text_component_result::PlainTextComponentResult,
        },
    },
    output::general::component_constants::component_names::EmbeddableComponentName,
    parsers::parser_trait::ConundrumParser,
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct EmojiResult {
    pub value: String,
}

impl EmojiResult {
    pub fn get_svg() {}
}

impl PlainTextComponentResult for EmojiResult {
    fn to_plain_text(&self,
                     _: &mut crate::lang::runtime::state::parse_state::ParseState)
                     -> ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}

impl MarkdownComponentResult for EmojiResult {
    fn to_markdown(&self,
                   res: &mut crate::lang::runtime::state::parse_state::ParseState)
                   -> ConundrumModalResult<String> {
        Ok(format!(":{}:", self.value))
    }
}

impl JsxComponentResult for EmojiResult {
    fn to_jsx_component(&self,
                        res: &mut crate::lang::runtime::state::parse_state::ParseState)
                        -> ConundrumModalResult<String> {
        // let svg = self.get_svg()
        format!("<{}>{}</{}>", EmbeddableComponentName::Emoji,)
    }
}

impl ConundrumComponentResult for EmojiResult {
    fn to_conundrum_component(&self,
                              res: &mut crate::lang::runtime::state::parse_state::ParseState)
                              -> ConundrumModalResult<String> {
        if res.contains_modifier(&ConundrumModifier::ForSearchInput) {
            self.to_plain_text(res)
        } else if res.is_markdown() {
            self.to_markdown(res)
        } else {
            self.to_jsx_component(res)
        }
    }
}

impl ConundrumParser<EmojiResult> for EmojiResult {
    fn parse_input_string(input: &mut crate::lang::runtime::traits::conundrum_input::ConundrumInput)
                          -> ConundrumModalResult<EmojiResult> {
        let value = delimited(':', take_while(1.., |c| !AsChar::is_space(c) && c != ':'), ':').parse_next(input)?;

        Ok(EmojiResult { value: value.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == ':'
    }
}
