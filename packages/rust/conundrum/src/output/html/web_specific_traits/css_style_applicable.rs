use crate::{
    lang::runtime::state::conundrum_error_variant::ConundrumModalResult,
    output::html::web_specific_models::css_style_properties::CSSProperties,
};

pub trait CssStyleApplicable {
    fn apply_styles(&self, styles: &mut CSSProperties) -> ConundrumModalResult<()>;
}
