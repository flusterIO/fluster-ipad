use serde::Serialize;

use crate::{
    lang::runtime::{
        state::conundrum_error_variant::ConundrumModalResult,
        traits::fluster_component_result::ConundrumComponentResult,
    },
    parsers::conundrum::logic::{
        bool::boolean::ConundrumBoolean, number::conundrum_number::ConundrumNumber,
        string::conundrum_string::ConundrumString,
    },
};

// All of the tokens that can appear in a Conundrum code block or .conundrum
// file, if one of those ever exists.
#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum ConundrumLogicToken {
    Number(ConundrumNumber),
    String(ConundrumString),
    Bool(ConundrumBoolean),
}

impl ConundrumComponentResult for ConundrumLogicToken {
    fn to_conundrum_component(&self,
                              res: &mut crate::lang::runtime::state::parse_state::ParseState)
                              -> ConundrumModalResult<String> {
        match self {
            ConundrumLogicToken::Number(n) => n.to_conundrum_component(res),
            ConundrumLogicToken::String(s) => s.to_conundrum_component(res),
            ConundrumLogicToken::Bool(b) => b.to_conundrum_component(res),
        }
    }
}
