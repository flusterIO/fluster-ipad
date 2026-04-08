use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            state::conundrum_error_variant::ConundrumModalResult,
            traits::{
                fluster_component_result::ConundrumComponentResult, markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::component_ids::EmbeddableComponentId,
    parsers::conundrum::logic::object::object::ConundrumObject,
};

/// A general trait that all user-embeddable components must implement in order
/// to be able to be derived from a props object. In other words, this is where
/// we read the properties that you provide to Conundrum and see if they make
/// sense.
pub trait ConundrumComponent: ConundrumComponentResult + MarkdownComponentResult + PlainTextComponentResult {
    fn get_component_id() -> EmbeddableComponentId;
    fn from_props(props: ConundrumObject, children: Option<Vec<ParsedElement>>) -> ConundrumModalResult<Self>
        where Self: Sized;
}
