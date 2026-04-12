use crate::lang::runtime::{
    mem::mem::{Mem, MemoryArc},
    state::conundrum_error_variant::ConundrumModalResult,
    traits::conundrum_input::ConundrumInput,
};
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

pub trait ConundrumLogicParser {
    /// The parser that operates on the logical side of Conundrum, inside of
    /// code blocks, the jsx superset and eventually in conundrum files, _not_
    /// in the markdown layer.
    /// This will surely need to be rewritten when I actually figure out
    /// what the heck I'm doing with this memory layer.
    fn parse_conundrum(input: &mut ConundrumInput, mem: &MemoryArc) -> ConundrumModalResult<Self>
        where Self: Sized;
}
