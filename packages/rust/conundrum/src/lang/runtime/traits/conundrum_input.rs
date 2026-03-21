use std::cell::RefCell;

use winnow::Stateful;

use crate::lang::runtime::state::parse_state::ParseState;

pub type ConundrumInput<'a> = Stateful<&'a str, RefCell<ParseState>>;
