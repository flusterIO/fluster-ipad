use crate::lang::runtime::{
    state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState},
    traits::conundrum_input::ArcState,
};

pub trait HtmlJsComponentResult {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String>;
}
