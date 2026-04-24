use serde::Serialize;

use crate::{
    lang::runtime::{
        state::conundrum_error_variant::ConundrumModalResult,
        traits::{conundrum_input::ArcState, fluster_component_result::ConundrumComponentResult},
    },
    parsers::conundrum::logic::{
        bool::boolean::ConundrumBoolean, function::conundrum_function::ConundrumFunction,
        number::conundrum_number::ConundrumNumber, object::object::ConundrumObject,
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
    Object(ConundrumObject),
    Function(ConundrumFunction),
}

impl ConundrumComponentResult for ConundrumLogicToken {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        match self {
            ConundrumLogicToken::Number(n) => n.to_conundrum_component(res),
            ConundrumLogicToken::String(s) => s.to_conundrum_component(res),
            ConundrumLogicToken::Bool(b) => b.to_conundrum_component(res),
            ConundrumLogicToken::Object(b) => b.to_conundrum_component(res),
            ConundrumLogicToken::Function(f) => f.to_conundrum_component(res),
        }
    }
}
