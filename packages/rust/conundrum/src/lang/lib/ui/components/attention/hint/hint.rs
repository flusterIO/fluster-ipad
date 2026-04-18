use serde::Serialize;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::component_trait::ConundrumComponent,
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
            ui_types::{children::Children, emphasis::Emphasis, heading_depth::HeadingDepth},
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::{ConundrumCompileTarget, ConundrumModifier, ParseState},
            },
            traits::{
                fluster_component_result::ConundrumComponentResult,
                inline_markdown_component_result::InlineMarkdownComponentResult,
                jsx_component_result::JsxComponentResult, markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult, plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_ids::EmbeddableComponentId,
        component_names::EmbeddableComponentName,
    },
    parsers::conundrum::logic::{
        object::object::ConundrumObject, string::conundrum_string::ConundrumString, token::ConundrumLogicToken,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Default, Clone)]
pub struct Hint {
    /// Note that `label` is a String and note a vector of children. This means
    /// that the fragment syntax won't work and markdown may or may not work in
    /// the label. Default: "hint".
    pub label: Option<ConundrumString>,
    pub children: Children,
    pub emphasis: Emphasis,
    pub markdown_title_depth: Option<HeadingDepth>,
}

impl Hint {
    pub fn get_label(&self) -> String {
        self.label.clone().map(|s| s.0).unwrap_or(String::from("hint"))
    }
}

impl MarkdownComponentResult for Hint {
    fn to_markdown(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(format!("**{}:** {}", self.get_label(), self.children.render(res)?))
    }
}

impl InlineMarkdownComponentResult for Hint {
    fn to_inline_markdown(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl PlainTextComponentResult for Hint {
    fn to_plain_text(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(format!("{}: {}", self.get_label(), self.children.render(res)?))
    }
}

impl JsxComponentResult for Hint {
    fn to_jsx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(format!("<{} label=\"{}\" {}>{}</{}>",
                   EmbeddableComponentName::Hint,
                   self.get_label(),
                   self.emphasis,
                   self.children.render(res)?,
                   EmbeddableComponentName::Hint))
    }
}

impl MdxComponentResult for Hint {
    fn to_mdx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.to_jsx_component(res)
    }
}

impl ConundrumComponentResult for Hint {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if res.contains_modifier(&ConundrumModifier::PreferInlineMarkdownSyntax) {
            self.to_inline_markdown(res)
        } else if res.compile_target == ConundrumCompileTarget::Markdown {
            self.to_markdown(res)
        } else if res.compile_target == ConundrumCompileTarget::PlainText {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl ConundrumComponent for Hint {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Hint)
    }

    fn from_props(props: ConundrumObject, children: Option<Vec<ParsedElement>>) -> ConundrumModalResult<Self>
        where Self: Sized {
        let label = props.data.get("label").map(|c| match c.value() {
                                               ParsedElement::Logic(s) => match s.clone() {
                                                   ConundrumLogicToken::String(s) => Some(s),
                                                   _ => None,
                                               }.ok_or(ConundrumErrorVariant::UserFacingMissingOrIncorrectProperty(ConundrumError::from_msg_and_details("Failed to parse the label property provided to the `Hint` component.", "Conundrum was looking for a `string`, but found another type."))),
                                               _ => Err(ConundrumErrorVariant::UserFacingMissingOrIncorrectProperty(ConundrumError::from_message("Failed to parse the label property provided to the `Hint` component."))),
                                           }).and_then(|x| {
                                            x.ok()
        });
        // .and_then(|n| {

        // });
        let emphasis = Emphasis::from_jsx_props(&props, "").unwrap_or(Emphasis::Success);
        let children = Children(children.unwrap_or_default());
        Ok(Hint { label,
                  children,
                  emphasis,
                  markdown_title_depth: None })
    }
}
