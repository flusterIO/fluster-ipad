use crate::lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ArcState};

pub trait HtmlJsComponentResult {
    fn to_html_js_component(&self, state: ArcState) -> ConundrumModalResult<String>;
}
