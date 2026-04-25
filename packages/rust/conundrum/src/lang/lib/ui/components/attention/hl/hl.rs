use askama::Template;
use serde::Serialize;
use typeshare::typeshare;
use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::{attention::hl::hl_html_templ::HlHtmlTemplate, component_trait::ConundrumComponent},
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
            ui_types::{
                children::Children, common_component_property_key::CommonComponentPropertyKey, emphasis::Emphasis,
                inline_markdown_override::InlineMarkdownOverride,
            },
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::ConundrumCompileTarget,
            },
            traits::{
                conundrum_input::ArcState, fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult, jsx_component_result::JsxComponentResult,
                markdown_component_result::MarkdownComponentResult, mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_names::EmbeddableComponentName,
    },
    parsers::conundrum::logic::object::object::ConundrumObject,
};

#[typeshare]
#[derive(Serialize, Debug, Clone)]
pub struct Highlight {
    pub children: Children,
    /// Default: .highlight
    pub emphasis: Emphasis,
    /// Control the markdown formatting when the output format is a markdown
    /// variant. This property is common throughout Conundrum, but the default
    /// is unique to each component.
    /// Default: `.Italic`
    pub markdown: Option<InlineMarkdownOverride>,
}

impl JsxComponentResult for Highlight {
    fn to_jsx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!(r#"<{} {}>{}</{}>"#,
                   EmbeddableComponentName::Hl,
                   self.emphasis,
                   self.children.render(res)?,
                   EmbeddableComponentName::Hl))
    }
}

impl HtmlJsComponentResult for Highlight {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let templ = HlHtmlTemplate { emphasis: self.emphasis.clone(),
                                     content: self.children.render(res)? };
        templ.render().map_err(|e| {
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl MdxComponentResult for Highlight {
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.to_jsx_component(res)
    }
}

impl MarkdownComponentResult for Highlight {
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(self.markdown
               .unwrap_or(InlineMarkdownOverride::BoldItalic)
               .wrap_content(self.children.render(res)?.as_str()))
    }
}

impl PlainTextComponentResult for Highlight {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl ConundrumComponentResult for Highlight {
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

impl ConundrumComponent for Highlight {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Hl)
    }

    fn from_props(props: ConundrumObject,
                  children: Option<Vec<ParsedElement>>,
                  _: ArcState)
                  -> ConundrumModalResult<Self>
        where Self: Sized {
        let emphasis = Emphasis::from_jsx_props(&props, "").unwrap_or(Emphasis::Highlight);
        let markdown =
            InlineMarkdownOverride::from_jsx_props(&props,
                                                   CommonComponentPropertyKey::InlineMarkdownOverride.to_string()
                                                                                                     .as_str()).ok();
        Ok(Highlight { children: Children(children.unwrap_or_default()),
                       emphasis,
                       markdown })
    }
}
