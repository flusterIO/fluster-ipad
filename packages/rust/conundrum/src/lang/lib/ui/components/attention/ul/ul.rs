use serde::Serialize;
use typeshare::typeshare;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::component_trait::ConundrumComponent,
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
            ui_types::{children::Children, emphasis::Emphasis, inline_markdown_override::InlineMarkdownOverride},
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
    parsers::conundrum::logic::object::object::ConundrumObject,
};

#[typeshare]
#[derive(Serialize, Debug, Clone)]
pub struct Underline {
    pub children: Children,
    /// Default: .highlight
    pub emphasis: Emphasis,
    /// Default: .Plain
    pub markdown: Option<InlineMarkdownOverride>,
}

impl JsxComponentResult for Underline {
    fn to_jsx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(format!(r#"<{} {}>{}</{}>"#,
                   EmbeddableComponentName::Ul,
                   self.emphasis,
                   self.children.render(res)?,
                   EmbeddableComponentName::Ul))
    }
}

impl MdxComponentResult for Underline {
    fn to_mdx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.to_jsx_component(res)
    }
}

impl MarkdownComponentResult for Underline {
    fn to_markdown(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(self.markdown.unwrap_or(InlineMarkdownOverride::Italic).wrap_content(self.children.render(res)?.as_str()))
    }
}

impl PlainTextComponentResult for Underline {
    fn to_plain_text(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl ConundrumComponentResult for Underline {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if res.contains_one_of_modifiers(vec![ConundrumModifier::PreferMarkdownSyntax,
                                              ConundrumModifier::PreferInlineMarkdownSyntax])
        {
            self.to_markdown(res)
        } else if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl ConundrumComponent for Underline {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Ul)
    }

    fn from_props(props: ConundrumObject, children: Option<Vec<ParsedElement>>) -> ConundrumModalResult<Self> {
        let markdown_output = InlineMarkdownOverride::from_jsx_props(&props, "markdown").ok();
        let emphasis = Emphasis::from_jsx_props(&props, "").unwrap_or(Emphasis::Highlight);
        Ok(Underline { children: Children(children.unwrap_or_default()),
                       emphasis,
                       markdown: markdown_output })
    }
}
