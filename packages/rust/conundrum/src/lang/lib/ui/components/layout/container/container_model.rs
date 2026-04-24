use askama::Template;
use serde::Serialize;
use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::{
                component_trait::ConundrumComponent,
                layout::container::container_html_templ::UtilityContainerHtmlTemplate,
            },
            shared_props::sizable::SizablePropsGroup,
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
            ui_types::{children::Children, emphasis::Emphasis},
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::ConundrumModifier,
            },
            traits::{
                conundrum_input::ArcState, fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult, jsx_component_result::JsxComponentResult,
                markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_names::EmbeddableComponentName,
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
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl PlainTextComponentResult for UtilityContainer {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl JsxComponentResult for UtilityContainer {
    fn to_jsx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!(
                   r#"<{}
{}
{}>
{}
</{}>"#,
                   EmbeddableComponentName::UtlityContainer,
                   self.sizable.to_jsx_prop(),
                   self.emphasis.as_ref().map(|x| x.to_string()).unwrap_or_else(|| String::from("")),
                   self.children.render(res)?,
                   EmbeddableComponentName::UtlityContainer,
        ))
    }
}

impl HtmlJsComponentResult for UtilityContainer {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let templ = UtilityContainerHtmlTemplate { children: self.children.render(res)?,
                                                   emphasis: self.emphasis.clone(),
                                                   sizable: self.sizable.clone() };
        templ.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl ConundrumComponentResult for UtilityContainer {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.contains_modifier(&ConundrumModifier::PreferInlineMarkdownSyntax) {
            drop(state);
            self.to_markdown(res)
        } else if state.contains_modifier(&ConundrumModifier::ForSearchInput) {
            drop(state);
            self.to_plain_text(res)
        } else {
            drop(state);
            self.to_jsx_component(res)
        }
    }
}

impl ConundrumComponent for UtilityContainer {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::UtlityContainer)
    }

    fn from_props(props: ConundrumObject,
                  children: Option<Vec<ParsedElement>>,
                  _: ArcState)
                  -> ConundrumModalResult<Self>
        where Self: Sized {
        let sizable = SizablePropsGroup::from_jsx_props(&props, "")?;
        let emphasis = Emphasis::from_jsx_props(&props, "").ok();
        Ok(UtilityContainer { sizable,
                              emphasis,
                              children: Children(children.unwrap_or_default()) })
    }
}
