use askama::Template;
use serde::Serialize;
use std::sync::Arc;
use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::{attention::hint::hint_html_templ::HintHtmlTemplate, component_trait::ConundrumComponent},
            shared_props::sizable::SizablePropsGroup,
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
            ui_types::{children::Children, emphasis::Emphasis, heading_depth::HeadingDepth},
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::{ConundrumCompileTarget, ConundrumModifier},
            },
            traits::{
                conundrum_input::ArcState, fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult,
                inline_markdown_component_result::InlineMarkdownComponentResult,
                jsx_component_result::JsxComponentResult, markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult, plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_names::EmbeddableComponentName,
    },
    parsers::conundrum::logic::{object::object::ConundrumObject, string::conundrum_string::ConundrumString},
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Default, Clone)]
pub struct Hint {
    /// Note that `label` is a String and note a vector of children. This means
    /// that the fragment syntax won't work and markdown may or may not work in
    /// the label. Default: "hint".
    pub label: Option<Children>,
    pub children: Children,
    pub emphasis: Emphasis,
    pub sizable: Option<SizablePropsGroup>,
    pub markdown_title_depth: Option<HeadingDepth>,
}

impl Hint {
    pub fn get_label(&self, res: ArcState) -> ConundrumModalResult<String> {
        if let Some(c) = &self.label {
            c.render(Arc::clone(&res)).map_err(|e| {
                        eprintln!("Error: {:#?}", e);
                        ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                    })
        } else {
            Ok(String::from("Hint"))
        }
    }
}

impl HtmlJsComponentResult for Hint {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let templ = HintHtmlTemplate { label: self.get_label(Arc::clone(&res))?,
                                       emphasis: self.emphasis.clone(),
                                       size: self.sizable.clone(),
                                       body: self.children.render(Arc::clone(&res))? };
        templ.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl MarkdownComponentResult for Hint {
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("**{}:** {}", self.get_label(Arc::clone(&res))?, self.children.render(Arc::clone(&res))?))
    }
}

impl InlineMarkdownComponentResult for Hint {
    fn to_inline_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl PlainTextComponentResult for Hint {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("{}: {}", self.get_label(Arc::clone(&res))?, self.children.render(Arc::clone(&res))?))
    }
}

impl JsxComponentResult for Hint {
    fn to_jsx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("<{} label=\"{}\" {}>{}</{}>",
                   EmbeddableComponentName::Hint,
                   self.get_label(Arc::clone(&res))?,
                   self.emphasis,
                   self.children.render(Arc::clone(&res))?,
                   EmbeddableComponentName::Hint))
    }
}

impl MdxComponentResult for Hint {
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.to_jsx_component(res)
    }
}

impl ConundrumComponentResult for Hint {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.contains_modifier(&ConundrumModifier::PreferInlineMarkdownSyntax) {
            drop(state);
            self.to_inline_markdown(res)
        } else if state.compile_target == ConundrumCompileTarget::Markdown {
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

impl ConundrumComponent for Hint {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Hint)
    }

    fn from_props(props: ConundrumObject,
                  children: Option<Vec<ParsedElement>>,
                  state: ArcState)
                  -> ConundrumModalResult<Self>
        where Self: Sized {
        let mut label_children: Option<Children> = None;
        if let Ok(s) = ConundrumString::from_jsx_props(&props, "label") {
            label_children = Some(s.to_children(state)?);
        }
        let emphasis = Emphasis::from_jsx_props(&props, "").unwrap_or(Emphasis::Success);
        let children = Children(children.unwrap_or_default());
        let sizable = SizablePropsGroup::from_jsx_props(&props, "").ok();
        Ok(Hint { label: label_children,
                  children,
                  emphasis,
                  sizable,
                  markdown_title_depth: None })
    }
}
