use crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult;

pub trait DisplayWithParam<T> {
    fn display_with_param(&self, args: T) -> ConundrumModalResult<String>;
}
