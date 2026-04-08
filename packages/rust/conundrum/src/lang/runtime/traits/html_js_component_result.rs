use crate::lang::runtime::state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState};

pub trait HtmlJsComponentResult {
    fn to_html_js_component(&self, res: &mut ParseState) -> ConundrumModalResult<String>;
}
