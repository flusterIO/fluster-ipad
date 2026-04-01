use crate::lang::runtime::state::parse_state::ParseState;

pub trait JsxComponentResult {
    fn to_jsx_component(&self, res: &mut ParseState) -> String;
}
