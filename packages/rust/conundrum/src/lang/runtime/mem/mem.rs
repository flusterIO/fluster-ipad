use std::cell::RefCell;
use std::sync::Arc;

use dashmap::DashMap;
use winnow::error::ErrMode;

use crate::{
    lang::runtime::state::{
        conundrum_error::ConundrumError,
        conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
    },
    parsers::conundrum::logic::token::ConundrumLogicToken,
};

/// **Do not** make the interior type public. Require that all modifications are
/// done through the methods below.
pub struct Mem(DashMap<String, ConundrumLogicToken>);

impl Mem {
    pub fn define(&mut self, key: String, value: ConundrumLogicToken) {
        self.0.insert(key, value);
    }

    pub fn get(&self, key: &str) -> ConundrumModalResult<ConundrumLogicToken> {
        self.0.get(key).map(|n| {
            n.value().clone()
        }).ok_or_else(|| {
            ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Unknown variable", format!("Conundrum could not find the `{}` variable. Are you sure it is defined?", key).as_str())))
        })
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }
}

pub type MemoryArc = Arc<RefCell<Mem>>;
