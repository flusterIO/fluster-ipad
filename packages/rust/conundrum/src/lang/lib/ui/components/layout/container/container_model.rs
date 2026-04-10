use serde::Serialize;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::component_trait::ConundrumComponent,
            shared_props::sizable::SizablePropsGroup,
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
            ui_types::{children::Children, emphasis::Emphasis},
        },
        runtime::{
            state::{
                conundrum_error_variant::ConundrumModalResult,
                parse_state::{ConundrumModifier, ParseState},
            },
            traits::{
                fluster_component_result::ConundrumComponentResult, jsx_component_result::JsxComponentResult,
                markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_ids::EmbeddableComponentId,
        component_names::EmbeddableComponentName,
    },
    parsers::conundrum::logic::object::object::ConundrumObject,
};

/// > Note that the `label` property can also be changed from it's default
/// > 'Hint' to any string.
///
///
/// The `Container` component is an intentionally almost entirely unstyled
/// component that accepts most of the _generic_ properties accepted elsewhere.
/// This means that it takes an optional `Emphasis` as a boolean, and all of the
/// `SizableObject` properties like `width`, `margin`, `padding`, `text` for
/// font size and a handful of positionable props. The goal with the `Container`
/// component is to provide a blank canvas to allow you to create some
/// responsive & informative layouts that fit your own use case.
#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct UtilityContainer {
    pub sizable: SizablePropsGroup,
    pub emphasis: Option<Emphasis>,
    pub children: Children,
}

impl MarkdownComponentResult for UtilityContainer {
    fn to_markdown(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl PlainTextComponentResult for UtilityContainer {
    fn to_plain_text(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl JsxComponentResult for UtilityContainer {
    fn to_jsx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(format!(
                   r#"<{}
{}
{}>
{}
</{}>"#,
                   EmbeddableComponentName::UtlityContainer,
                   self.sizable.to_jsx_prop(),
                   self.emphasis.as_ref().unwrap_or(&Emphasis::Card),
                   self.children.render(res)?,
                   EmbeddableComponentName::UtlityContainer,
        ))
    }
}

impl ConundrumComponentResult for UtilityContainer {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if res.contains_one_of_modifiers(vec![ConundrumModifier::PreferMarkdownSyntax,
                                              ConundrumModifier::PreferInlineMarkdownSyntax])
        {
            self.to_markdown(res)
        } else if res.contains_one_of_modifiers(vec![ConundrumModifier::ForcePlainText,
                                                     ConundrumModifier::ForSearchInput,
                                                     ConundrumModifier::ForSearchInput])
        {
            self.to_plain_text(res)
        } else {
            self.to_jsx_component(res)
        }
    }
}

impl ConundrumComponent for UtilityContainer {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::UtlityContainer)
    }

    fn from_props(props: ConundrumObject, children: Option<Vec<ParsedElement>>) -> ConundrumModalResult<Self>
        where Self: Sized {
        let sizable = SizablePropsGroup::from_jsx_props(&props, "")?;
        let emphasis = Emphasis::from_jsx_props(&props, "").ok();
        Ok(UtilityContainer { sizable,
                              emphasis,
                              children: Children(children.unwrap_or(Vec::new())) })
    }
}
