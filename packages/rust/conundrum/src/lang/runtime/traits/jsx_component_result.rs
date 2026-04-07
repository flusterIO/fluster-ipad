use crate::lang::runtime::state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState};

pub trait JsxComponentResult {
    fn to_jsx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String>;
}
