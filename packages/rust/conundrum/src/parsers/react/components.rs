use std::{cell::RefCell, sync::Arc};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::components::{
            academic::equation_reference::equation_reference_model::EquationReference,
            attention::{admonition::admonition::Admonition, hint::hint::Hint, hl::hl::Highlight, ul::ul::Underline},
            component_trait::ConundrumComponent,
            documentation::emoji::emoji_docs_demo::EmojiDocsDemo,
            layout::{
                card::card::Card,
                container::container_model::UtilityContainer,
                grid::grid::ResponsiveGrid,
                tabs::{tabs_group::TabsGroup, tabs_group_tab::Tab},
                toc::table_of_contents::TableOfContents,
            },
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::ParseState,
            },
            traits::conundrum_input::ArcState,
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_ids::EmbeddableComponentId,
        documentation_component_name::DocumentationComponentName,
    },
    parsers::{
        conundrum::{
            hr_with_children::hr_with_children_model::HrWithChildrenResult, logic::object::object::ConundrumObject,
        },
        markdown::markdown_extensions::emoji::EmojiResult,
        react::conundrum_component::ConundrumComponentType,
    },
};
use dashmap::DashMap;
use lazy_static::lazy_static;
use winnow::error::ErrMode;

pub type ComponentGetter =
    Box<dyn Fn(ConundrumObject, Option<Vec<ParsedElement>>, ArcState) -> ConundrumModalResult<ConundrumComponentType>
            + Send
            + Sync>;

pub type ComponentMap = DashMap<String, ComponentGetter>;

lazy_static! {
    pub static ref COMPONENT_MAP: ComponentMap = {
        let m: ComponentMap = DashMap::new();
        m.insert(EmbeddableComponentId::Card.to_string(),
                 Box::new(|props, children, state| {
                     Card::from_props(props, children, state).map(ConundrumComponentType::Card)
                 }));
        m.insert(EmbeddableComponentId::Admonition.to_string(),
                 Box::new(|props, children, state| {
                     Admonition::from_props(props, children, state).map(ConundrumComponentType::Admonition)
                 }));
        m.insert(EmbeddableComponentId::Hint.to_string(),
                 Box::new(|props, children, state| {
                     Hint::from_props(props, children, state).map(ConundrumComponentType::Hint)
                 }));
        m.insert(EmbeddableComponentId::Ul.to_string(),
                 Box::new(|props, children, state| {
                     Underline::from_props(props, children, state).map(ConundrumComponentType::Ul)
                 }));
        m.insert(EmbeddableComponentId::Hl.to_string(),
                 Box::new(|props, children, state| {
                     Highlight::from_props(props, children, state).map(ConundrumComponentType::Hl)
                 }));
        m.insert(EmbeddableComponentId::Tab.to_string(),
                 Box::new(|props, children, state| {
                     Tab::from_props(props, children, state).map(ConundrumComponentType::Tab)
                 }));
        m.insert(EmbeddableComponentId::Tabs.to_string(),
                 Box::new(|props, children, state| {
                     TabsGroup::from_props(props, children, state).map(ConundrumComponentType::Tabs)
                 }));
        m.insert(EmbeddableComponentId::EqRef.to_string(),
                 Box::new(|props, children, state| {
                     EquationReference::from_props(props, children, state).map(ConundrumComponentType::EqRef)
                 }));
        m.insert(EmbeddableComponentId::HrWithChildren.to_string(),
                 Box::new(|props, children, state| {
                     HrWithChildrenResult::from_props(props, children, state).map(ConundrumComponentType::Hr)
                 }));
        m.insert(EmbeddableComponentId::UtlityContainer.to_string(),
                 Box::new(|props, children, state| {
                     UtilityContainer::from_props(props, children, state).map(ConundrumComponentType::Container)
                 }));
        m.insert(EmbeddableComponentId::Grid.to_string(),
                 Box::new(|props, children, state| {
                     ResponsiveGrid::from_props(props, children, state).map(ConundrumComponentType::Grid)
                 }));
        m.insert(EmbeddableComponentId::Emoji.to_string(),
                 Box::new(|props, children, state| {
                     EmojiResult::from_props(props, children, state).map(ConundrumComponentType::Emoji)
                 }));
        m.insert(DocumentationComponentName::EmojiDocumentationDemo.to_string(),
                 Box::new(|props, children, state| {
                     EmojiDocsDemo::from_props(props, children, state).map(ConundrumComponentType::EmojiDocsDemo)
                 }));
        m.insert(EmbeddableComponentId::Toc.to_string(),
                 Box::new(|props, children, state| {
                     TableOfContents::from_props(props, children, state).map(ConundrumComponentType::Toc)
                 }));
        m
    };
}

impl COMPONENT_MAP {
    pub fn get_by_component_name(&self,
                                 id: &AnyComponentName,
                                 props: ConundrumObject,
                                 children: Option<Vec<ParsedElement>>,
                                 state: ArcState)
                                 -> ConundrumModalResult<ConundrumComponentType> {
        let component_name_string = match id {
            AnyComponentName::UserEmbedded(u) => u.to_component_id().to_string(),
            AnyComponentName::Docs(d) => d.to_string(),
            AnyComponentName::AutoInserted(a) => a.to_string(),
        };
        let res = self.get(&component_name_string);
        match res {
            Some(s) => {
                let f = s.value();
                let r = f(props, children, state)?;
                Ok(r)
            }
            None => {
                Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Missing Component",
                                                                                                                   "The component key you provided could not be found."))))
            }
        }
    }
}
