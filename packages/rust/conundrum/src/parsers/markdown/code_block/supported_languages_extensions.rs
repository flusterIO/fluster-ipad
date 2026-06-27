use strum::IntoEnumIterator;
use winnow::error::ErrMode;

use crate::{
    lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant,
    parsers::{
        conundrum::logic::string::conundrum_string::ConundrumString,
        markdown::code_block::supported_languages::SupportedCodeBlockSyntax,
    },
};

impl TryFrom<ConundrumString> for SupportedCodeBlockSyntax {
    type Error = ErrMode<ConundrumErrorVariant>;

    fn try_from(value: ConundrumString) -> Result<Self, Self::Error> {
        for item in SupportedCodeBlockSyntax::iter() {
            if item.to_string() == value.0 {
                return Ok(item);
            }
        }
        Err(ErrMode::Backtrack(ConundrumErrorVariant::TypeConversionError { from: String::from("string"),
                                                                            to: String::from("SupportedCodeBlockSyntax") }))
    }
}
