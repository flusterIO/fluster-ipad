use crate::{
    lang::runtime::state::conundrum_error_variant::ConundrumModalResult,
    output::general::component_constants::component_ids::EmbeddableComponentId,
};

/// A simple trait to implement a function that gathers whatever gue code is
/// needed for the component to function and returns it so it can be embedded in
/// the html file when the output is in standalone mode.
///
/// When the output is not in standalone mode an independent javascript and css
/// file should be prefered so it's not re-read every time the html changes.
pub trait ComponentBrowserGlueGetter {
    fn get_component_glue(id: EmbeddableComponentId) -> ConundrumModalResult<String>;
}
