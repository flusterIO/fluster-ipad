use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::component_trait::ConundrumComponent, ui_traits::jsx_prop_representable::FromJsxPropsOptional,
            ui_types::children::Children,
        },
        runtime::{
            state::{conundrum_error_variant::ConundrumModalResult, parse_state::ConundrumCompileTarget},
            traits::{
                conundrum_input::ArcState, fluster_component_result::ConundrumComponentResult,
                jsx_component_result::JsxComponentResult, markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult, plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_names::EmbeddableComponentName,
    },
    parsers::conundrum::logic::{
        bool::boolean::ConundrumBoolean, object::object::ConundrumObject, string::conundrum_string::ConundrumString,
    },
};
use std::sync::Arc;
use winnow::error::ErrMode;

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
    pub initial: Option<ConundrumBoolean>,
}

impl PlainTextComponentResult for Tab {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl MdxComponentResult for Tab {
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.to_jsx_component(res)
    }
}

// impl HtmlJsComponentResult for Tab {
//     fn to_html_js_component(&self, res: ArcState) ->
// ConundrumModalResult<String> {         TabButtonHtmlTemplate
//     }
// }

impl ConundrumComponentResult for Tab {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.targets_markdown() {
            drop(state);
            self.to_markdown(res)
        } else if state.compile_target == ConundrumCompileTarget::PlainText {
            drop(state);
            self.to_plain_text(res)
        } else {
            drop(state);
            self.to_mdx_component(res)
        }
    }
}

impl ConundrumComponent for Tab {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Tab)
    }

    fn from_props(props: ConundrumObject,
                  children: Option<Vec<ParsedElement>>,
                  _: ArcState)
                  -> ConundrumModalResult<Self> {
        let label = ConundrumString::from_jsx_props(&props, "label").map_err(|e| e.cut())?;
        let id = ConundrumString::from_jsx_props(&props, "id").ok();
        let initial = ConundrumBoolean::from_jsx_props(&props, "initial").ok();
        let children = Children(children.unwrap_or_default());
        Ok(Tab { label,
                 id,
                 initial,
                 children })
    }
}

impl JsxComponentResult for Tab {
    fn to_jsx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let label_children = self.label.to_children(res.clone())?;
        let j = label_children.to_jsx_prop("label", Arc::clone(&res))?;
        let mut props = vec![j];
        let label_res = label_children.to_jsx_prop_as_string("labelString", Arc::clone(&res))?;
        props.push(label_res);
        if let Some(id) = &self.id {
            props.push(id.to_jsx_prop_as_string("id").map_err(ErrMode::Backtrack)?)
        }
        let children_string = self.children.render(Arc::clone(&res))?;
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
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}
