use crate::lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ArcState};

/// This guy got kind lost in the mix to the
/// [MdxComponentResult](crate::lang::runtime::traits::mdx_component_result::MdxComponentResult).
/// You should probably check that out if you're planning to render Conundrum
/// content in a React environment.
///
/// Beware though, this is still _super_ early in Conundrum's life, and it'll be
/// another month or two before all of the React templates are back in working
/// order after Fluster moved to a html & js target.
pub trait JsxComponentResult {
    fn to_jsx_component(&self, res: ArcState) -> ConundrumModalResult<String>;
}
