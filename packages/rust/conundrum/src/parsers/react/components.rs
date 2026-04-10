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
        runtime::state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
        },
    },
    output::general::component_constants::{
        auto_inserted_component_name::AutoInsertedComponentName, component_ids::EmbeddableComponentId,
    },
    parsers::{
        conundrum::{hr_with_children::HrWithChildrenResult, logic::object::object::ConundrumObject},
        markdown::markdown_extensions::emoji::EmojiResult,
        react::conundrum_component::ConundrumComponentType,
    },
};
use dashmap::DashMap;
use lazy_static::lazy_static;
use winnow::error::ErrMode;

pub type ComponentGetter =
    Box<dyn Fn(ConundrumObject, Option<Vec<ParsedElement>>) -> ConundrumModalResult<ConundrumComponentType>
            + Send
            + Sync>;

pub type ComponentMap = DashMap<String, ComponentGetter>;

lazy_static! {
    pub static ref COMPONENT_MAP: ComponentMap = {
        let m: ComponentMap = DashMap::new();
        m.insert(EmbeddableComponentId::Card.to_string(),
                 Box::new(|props, children| Card::from_props(props, children).map(ConundrumComponentType::Card)));

        m.insert(EmbeddableComponentId::Admonition.to_string(),
                 Box::new(|props, children| {
                     Admonition::from_props(props, children).map(ConundrumComponentType::Admonition)
                 }));
        m.insert(EmbeddableComponentId::Hint.to_string(),
                 Box::new(|props, children| Hint::from_props(props, children).map(ConundrumComponentType::Hint)));
        m.insert(EmbeddableComponentId::Ul.to_string(),
                 Box::new(|props, children| Underline::from_props(props, children).map(ConundrumComponentType::Ul)));
        m.insert(EmbeddableComponentId::Hl.to_string(),
                 Box::new(|props, children| Highlight::from_props(props, children).map(ConundrumComponentType::Hl)));
        m.insert(EmbeddableComponentId::Tab.to_string(),
                 Box::new(|props, children| Tab::from_props(props, children).map(ConundrumComponentType::Tab)));
        m.insert(EmbeddableComponentId::Tabs.to_string(),
                 Box::new(|props, children| TabsGroup::from_props(props, children).map(ConundrumComponentType::Tabs)));
        m.insert(EmbeddableComponentId::EqRef.to_string(),
                 Box::new(|props, children| {
                     EquationReference::from_props(props, children).map(ConundrumComponentType::EqRef)
                 }));
        m.insert(EmbeddableComponentId::HrWithChildren.to_string(),
                 Box::new(|props, children| {
                     HrWithChildrenResult::from_props(props, children).map(ConundrumComponentType::Hr)
                 }));
        m.insert(EmbeddableComponentId::UtlityContainer.to_string(),
                 Box::new(|props, children| {
                     UtilityContainer::from_props(props, children).map(ConundrumComponentType::Container)
                 }));
        m.insert(EmbeddableComponentId::Grid.to_string(),
                 Box::new(|props, children| {
                     ResponsiveGrid::from_props(props, children).map(ConundrumComponentType::Grid)
                 }));
        m.insert(EmbeddableComponentId::Emoji.to_string(),
                 Box::new(|props, children| {
                     EmojiResult::from_props(props, children).map(ConundrumComponentType::Emoji)
                 }));
        m
    };
}

impl COMPONENT_MAP {
    pub fn get_by_component_id(&self,
                               id: &EmbeddableComponentId,
                               props: ConundrumObject,
                               children: Option<Vec<ParsedElement>>)
                               -> ConundrumModalResult<ConundrumComponentType> {
        let res = self.get(&id.to_string());
        match res {
            Some(s) => {
                let f = s.value();
                let r = f(props, children)?;
                Ok(r)
            }
            None => {
                Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Missing Component",
                                                                                                                   "The component key you provided could not be found."))))
            }
        }
    }
}
