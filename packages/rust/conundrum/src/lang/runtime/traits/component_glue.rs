use crate::{
    lang::runtime::state::conundrum_error_variant::ConundrumModalResult,
    output::general::component_constants::component_ids::EmbeddableComponentId,
};

pub trait ComponentBrowserGlueGetter {
    fn get_component_glue(id: EmbeddableComponentId) -> ConundrumModalResult<String>;
}
