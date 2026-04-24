use winnow::Parser;
use winnow::combinator::alt;

use crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult;
use crate::lang::runtime::traits::conundrum_input::ConundrumInput;
use crate::parsers::conundrum::conundrum_logic_parser::ConundrumLogicParser;
use crate::parsers::conundrum::logic::bool::boolean::ConundrumBoolean;
use crate::parsers::conundrum::logic::function::conundrum_function::ConundrumFunction;
use crate::parsers::conundrum::logic::number::conundrum_number::ConundrumNumber;
use crate::parsers::conundrum::logic::object::object::ConundrumObject;
use crate::parsers::conundrum::logic::string::conundrum_string::ConundrumString;
use crate::parsers::conundrum::logic::token::ConundrumLogicToken;
/// - [ ] Move this to a return type of `ConundrumLogicToken` and replace the
///   number parser so that it catches ints properly.
pub fn javascript_object_value(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumLogicToken> {
    alt((ConundrumString::parse_conundrum.map(ConundrumLogicToken::String),
         ConundrumFunction::parse_conundrum.map(ConundrumLogicToken::Function),
         ConundrumObject::parse_conundrum.map(ConundrumLogicToken::Object),
         ConundrumNumber::parse_conundrum.map(ConundrumLogicToken::Number),
         ConundrumBoolean::parse_conundrum.map(ConundrumLogicToken::Bool))).parse_next(input)
}
