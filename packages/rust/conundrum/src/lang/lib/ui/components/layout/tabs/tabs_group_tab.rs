use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::component_trait::ConundrumComponent, ui_traits::jsx_prop_representable::FromJsxPropsOptional,
            ui_types::children::Children,
        },
        runtime::{
            state::{
                conundrum_error_variant::ConundrumModalResult,
                parse_state::{ConundrumModifier, ParseState},
            },
            traits::{
                fluster_component_result::ConundrumComponentResult, jsx_component_result::JsxComponentResult,
                markdown_component_result::MarkdownComponentResult, mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_ids::EmbeddableComponentId,
        component_names::EmbeddableComponentName,
    },
    parsers::conundrum::logic::{object::object::ConundrumObject, string::conundrum_string::ConundrumString},
};

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone)]
pub struct Tab {
    /// The label of the button that toggles this tab.
    pub label: ConundrumString,
    /// This is only required if the `label` field occurs more than once in the
    /// same `Tabs` component. Each `Tab` must have a unique `id` field, it's
    /// just set to the label by default.
    pub id: Option<ConundrumString>,
    pub children: Children,
}

impl PlainTextComponentResult for Tab {
    fn to_plain_text(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl MdxComponentResult for Tab {
    fn to_mdx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.to_jsx_component(res)
    }
}

impl ConundrumComponentResult for Tab {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if res.is_markdown() {
            self.to_markdown(res)
        } else if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl ConundrumComponent for Tab {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Tab)
    }

    fn from_props(props: ConundrumObject, children: Option<Vec<ParsedElement>>) -> ConundrumModalResult<Self> {
        let label = ConundrumString::from_jsx_props(&props, "label").map_err(|e| e.cut())?;
        let id = ConundrumString::from_jsx_props(&props, "id").ok();
        let children = Children(children.unwrap_or_default());
        Ok(Tab { label,
                 id,
                 children })
    }
}

impl JsxComponentResult for Tab {
    fn to_jsx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        let label_children = self.label.to_children(res.modifiers.clone())?;
        let j = label_children.to_jsx_prop("label", res)?;
        let mut props = vec![j];
        let label_res = label_children.to_jsx_prop_as_string("labelString", res)?;
        props.push(label_res);
        if let Some(id) = &self.id {
            props.push(id.to_jsx_prop_as_string("id").map_err(ErrMode::Backtrack)?)
        }
        let children_string = self.children.render(res)?;
        Ok(format!(
                   r#"<{} {}>
{}
</{}>"#,
                   EmbeddableComponentName::Tab,
                   props.join(" "),
                   children_string,
                   EmbeddableComponentName::Tab,
        ))
    }
}

impl MarkdownComponentResult for Tab {
    fn to_markdown(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}
