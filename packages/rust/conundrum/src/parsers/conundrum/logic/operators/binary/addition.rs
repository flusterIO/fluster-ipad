use winnow::ModalResult;

use crate::{
    lang::runtime::{
        state::conundrum_error_variant::{ConundrumModalResult, ConundrumResult},
        traits::conundrum_input::ConundrumInput,
    },
    parsers::{conundrum::logic::token::ConundrumLogicToken, parser_trait::ConundrumParser},
};

pub struct AdditionOperator {
    pub prefix: ConundrumLogicToken,
    pub suffix: ConundrumLogicToken,
}

impl ConundrumParser<AdditionOperator> for AdditionOperator {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<AdditionOperator> {
        todo!()
    }

    fn matches_first_char(char: char) -> bool {
        todo!()
    }
}
