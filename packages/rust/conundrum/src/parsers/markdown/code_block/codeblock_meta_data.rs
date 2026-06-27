use std::{str::FromStr, sync::Arc};

use parking_lot::RwLock;
use winnow::error::ErrMode;

use crate::{
    lang::runtime::{
        state::{conundrum_error_variant::ConundrumErrorVariant, parse_state::ParseState},
        traits::conundrum_input::ConundrumInput,
    },
    parsers::conundrum::logic::object::object::ConundrumObject,
};

/// Beginning to move this to it's own object because it's becoming a pain in
/// the axx.
#[derive(Clone)]
pub struct CodeBlockMetaData(pub ConundrumObject);

impl FromStr for CodeBlockMetaData {
    type Err = ErrMode<ConundrumErrorVariant>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = &mut ConundrumInput { input: s,
                                          state: Arc::new(RwLock::new(ParseState::default())) };
        ConundrumObject::from_single_line_property_string_parser(input).map(|obj| CodeBlockMetaData(obj))
    }
}
