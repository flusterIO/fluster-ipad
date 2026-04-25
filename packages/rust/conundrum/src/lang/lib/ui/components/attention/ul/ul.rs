use askama::Template;
use serde::Serialize;
use typeshare::typeshare;
use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::{attention::ul::ul_html_template::UlHtmlTemplate, component_trait::ConundrumComponent},
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
            ui_types::{children::Children, emphasis::Emphasis, inline_markdown_override::InlineMarkdownOverride},
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
    parsers::conundrum::logic::{bool::boolean::ConundrumBoolean, object::object::ConundrumObject},
};

#[typeshare]
#[derive(Serialize, Debug, Clone)]
pub struct Underline {
    pub children: Children,
    /// Default: .highlight
    pub emphasis: Emphasis,
    pub thin: Option<ConundrumBoolean>,
    pub thick: Option<ConundrumBoolean>,
    /// Default: .Plain
    pub markdown: Option<InlineMarkdownOverride>,
}

impl HtmlJsComponentResult for Underline {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let templ = UlHtmlTemplate { thin: self.thin,
                                     thick: self.thick,
                                     emphasis: self.emphasis.clone(),
                                     content: self.children.render(res)? };
        templ.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl JsxComponentResult for Underline {
    fn to_jsx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!(r#"<{} {}>{}</{}>"#,
                   EmbeddableComponentName::Ul,
                   self.emphasis,
                   self.children.render(res)?,
                   EmbeddableComponentName::Ul))
    }
}

impl MdxComponentResult for Underline {
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.to_jsx_component(res)
    }
}

impl MarkdownComponentResult for Underline {
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(self.markdown.unwrap_or(InlineMarkdownOverride::Italic).wrap_content(self.children.render(res)?.as_str()))
    }
}

impl PlainTextComponentResult for Underline {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl ConundrumComponentResult for Underline {
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

impl ConundrumComponent for Underline {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Ul)
    }

    fn from_props(props: ConundrumObject,
                  children: Option<Vec<ParsedElement>>,
                  _: ArcState)
                  -> ConundrumModalResult<Self> {
        let markdown_output = InlineMarkdownOverride::from_jsx_props(&props, "markdown").ok();
        let emphasis = Emphasis::from_jsx_props(&props, "").unwrap_or(Emphasis::Highlight);
        let thin = ConundrumBoolean::from_jsx_props(&props, "thin").ok();
        let thick = ConundrumBoolean::from_jsx_props(&props, "thick").ok();
        Ok(Underline { children: Children(children.unwrap_or_default()),
                       emphasis,
                       thin,
                       thick,
                       markdown: markdown_output })
    }
}
