use crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult;

pub trait ToStringReactCasing {
    fn to_string_react_casing(&self) -> ConundrumModalResult<String>;
}
