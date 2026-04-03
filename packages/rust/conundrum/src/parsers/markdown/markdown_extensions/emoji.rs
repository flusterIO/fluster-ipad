use serde::Serialize;
use winnow::{Parser, combinator::delimited, stream::AsChar, token::take_while};

use crate::{
    lang::runtime::{
        state::{conundrum_error_variant::ConundrumResult, parse_state::ConundrumModifier},
        traits::{
            fluster_component_result::ConundrumComponentResult, mdx_component_result::MdxComponentResult,
            plain_text_component_result::PlainTextComponentResult,
        },
    },
    parsers::parser_trait::ConundrumParser,
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct EmojiResult {
    pub value: String,
}

impl PlainTextComponentResult for EmojiResult {
    fn to_plain_text(&self, _: &mut crate::lang::runtime::state::parse_state::ParseState) -> String {
        String::from("")
    }
}

impl MdxComponentResult for EmojiResult {
    fn to_mdx_component(&self, _: &mut crate::lang::runtime::state::parse_state::ParseState) -> String {
        format!(":{}:", self.value)
    }
}

impl ConundrumComponentResult for EmojiResult {
    fn to_conundrum_component(&self, res: &mut crate::lang::runtime::state::parse_state::ParseState) -> String {
        if res.contains_modifier(&ConundrumModifier::ForSearchInput) {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl ConundrumParser<EmojiResult> for EmojiResult {
    fn parse_input_string(input: &mut crate::lang::runtime::traits::conundrum_input::ConundrumInput)
                          -> ConundrumResult<EmojiResult> {
        let value = delimited(':', take_while(1.., |c| !AsChar::is_space(c) && c != ':'), ':').parse_next(input)?;

        Ok(EmojiResult { value: value.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == ':'
    }
}
