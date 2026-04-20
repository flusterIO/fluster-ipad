use crate::lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ArcState};

pub trait JsxComponentResult {
    fn to_jsx_component(&self, res: ArcState) -> ConundrumModalResult<String>;
}
