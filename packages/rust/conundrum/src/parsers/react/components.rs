use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::components::{
            academic::equation_reference::equation_reference_model::EquationReference,
            attention::{admonition::admonition::Admonition, hint::hint::Hint, hl::hl::Highlight, ul::ul::Underline},
            component_trait::ConundrumComponent,
            layout::{
                card::card::Card,
                container::container_model::UtilityContainer,
                grid::grid::ResponsiveGrid,
                tabs::{tabs_group::TabsGroup, tabs_group_tab::Tab},
            },
        },
        runtime::state::conundrum_error_variant::ConundrumModalResult,
    },
    output::general::component_constants::component_ids::EmbeddableComponentId,
    parsers::{conundrum::logic::object::object::ConundrumObject, react::conundrum_component::ConundrumComponentType},
};
use dashmap::DashMap;
use lazy_static::lazy_static;

pub type ComponentGetter =
    Box<dyn Fn(ConundrumObject, Option<Vec<ParsedElement>>) -> ConundrumModalResult<ConundrumComponentType>
            + Send
            + Sync>;

pub type ComponentMap = DashMap<EmbeddableComponentId, ComponentGetter>;

lazy_static! {
    pub static ref COMPONENT_MAP: ComponentMap = {
        let m: ComponentMap = DashMap::new();
        m.insert(EmbeddableComponentId::Card,
                 Box::new(|props, children| Card::from_props(props, children).map(ConundrumComponentType::Card)));

        m.insert(EmbeddableComponentId::Admonition,
                 Box::new(|props, children| {
                     Admonition::from_props(props, children).map(ConundrumComponentType::Admonition)
                 }));
        m.insert(EmbeddableComponentId::Hint,
                 Box::new(|props, children| Hint::from_props(props, children).map(ConundrumComponentType::Hint)));
        m.insert(EmbeddableComponentId::Ul,
                 Box::new(|props, children| Underline::from_props(props, children).map(ConundrumComponentType::Ul)));
        m.insert(EmbeddableComponentId::Hl,
                 Box::new(|props, children| Highlight::from_props(props, children).map(ConundrumComponentType::Hl)));
        m.insert(EmbeddableComponentId::Tab,
                 Box::new(|props, children| Tab::from_props(props, children).map(ConundrumComponentType::Tab)));
        m.insert(EmbeddableComponentId::Tabs,
                 Box::new(|props, children| TabsGroup::from_props(props, children).map(ConundrumComponentType::Tabs)));
        m.insert(EmbeddableComponentId::EqRef,
                 Box::new(|props, children| {
                     EquationReference::from_props(props, children).map(ConundrumComponentType::EqRef)
                 }));
        m.insert(EmbeddableComponentId::UtlityContainer,
                 Box::new(|props, children| {
                     UtilityContainer::from_props(props, children).map(ConundrumComponentType::Container)
                 }));
        m.insert(EmbeddableComponentId::Grid,
                 Box::new(|props, children| {
                     ResponsiveGrid::from_props(props, children).map(ConundrumComponentType::Grid)
                 }));
        m
    };
}
