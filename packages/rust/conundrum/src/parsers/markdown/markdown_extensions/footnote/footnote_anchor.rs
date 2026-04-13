use serde::Serialize;
use winnow::{Parser, combinator::{delimited, preceded}, error::ErrMode};

use crate::{lang::runtime::{state::{conundrum_error::ConundrumError, conundrum_error_variant::{ConundrumErrorVariant}}, traits::conundrum_input::ConundrumInput}, parsers::{conundrum::logic::number::{conundrum_int::ConundrumInt, conundrum_number::ConundrumNumber}, javascript::javascript_number::{javascript_int}, parser_components::white_space_delimited::white_space_delimited, parser_trait::ConundrumParser}};

/// The `[^1]` syntax that goes in the markdown content, _not_ the footnote that
/// goes in the footer.
#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct FootNoteAnchor {
    pub idx: ConundrumInt,
}

impl ConundrumParser<FootNoteAnchor> for FootNoteAnchor {
    fn parse_input_string(input: &mut crate::lang::runtime::traits::conundrum_input::ConundrumInput) -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<FootNoteAnchor> {
        let idx = delimited(
            white_space_delimited(|input: &mut ConundrumInput| {
        '['.parse_next(input)
    }), 
            preceded('^', javascript_int), white_space_delimited(|input: &mut ConundrumInput| {
        ']'.parse_next(input)
    })).verify_map(|c| {
            match c.value {
                ConundrumNumber::Float(_) => {
                        None
                },
                ConundrumNumber::Int(n) => Some(n)
            }
        }).parse_next(input).map_err(|_| {
                    ErrMode::Cut(
                        ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid input", "Conundrum expected an integer at for a footnote index but received a float.")))
                    
            })?;

        Ok(FootNoteAnchor {
            idx
        })

    }

    fn matches_first_char(char: char) -> bool {
        char == '['
    }
}
