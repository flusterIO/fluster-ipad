use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::components::{
            attention::{admonition::admonition::Admonition, hint::hint::Hint, hl::hl::Highlight, ul::ul::Underline},
            component_trait::ConundrumComponent,
            layout::{
                card::card::Card,
                tabs::{tabs_group::TabsGroup, tabs_group_tab::Tab},
            },
        },
        runtime::state::conundrum_error_variant::{ConundrumModalResult, ConundrumResult},
    },
    output::general::component_constants::component_ids::EmbeddableComponentId,
    parsers::{conundrum::logic::object::object::ConundrumObject, react::conundrum_component::ConundrumComponentType},
};
use dashmap::DashMap;
use lazy_static::lazy_static;

// static COMPONENT_LIST: Lazy<DashMap<ConundrumComponentId,
// FnOnce(ConundrumObject) -> ConundrumComponentType>> =     Lazy::new(|| {
//         let data: DashMap<ConundrumComponentId, impl FnOnce(ConundrumObject)
// -> ConundrumComponentType> =             DashMap::new();

//         data.insert(ConundrumComponentId::Card, Card::from_props);

//         data
//     });
//

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
        m
    };
}
