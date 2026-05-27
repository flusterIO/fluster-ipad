use std::sync::Arc;

use askama::Template;
use serde::Serialize;
use typeshare::typeshare;
use winnow::error::ErrMode;

use crate::{
    lang::{
        lib::ui::{
            components::{
                attention::quote::{
                    quote_html_templ::QuoteHtmlTemplate, quote_markdown_option_variants::QuoteMarkdownVariants,
                },
                component_trait::ConundrumComponent,
            },
            shared_props::{
                markdown_component_options::markdown_component_options_model::MarkdownComponentOptions,
                sizable::SizablePropsGroup,
            },
            ui_traits::jsx_prop_representable::{FromJsxPropsOptional, FromJsxPropsOptionalWithState},
            ui_types::{children::Children, common_component_property_key::CommonComponentPropertyKey},
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            },
            traits::{
                conundrum_input::ArcState, fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult, markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_names::EmbeddableComponentName,
    },
    parsers::conundrum::logic::string::conundrum_string::ConundrumString,
};

#[typeshare]
#[derive(Debug, Serialize, serde::Deserialize, Clone)]
pub struct Quote {
    /// The primary content of the quote
    pub content: Children,
    /// An optional source of the quote
    pub source: Option<Children>,
    /// An optional `sourceId` that can be used to group quotes throughout
    /// a user's collection of notes based on where they appear.
    pub source_id: Option<ConundrumString>,
    pub sizable: SizablePropsGroup,
    pub markdown_options: Option<MarkdownComponentOptions<QuoteMarkdownVariants>>,
}

impl ConundrumComponentResult for Quote {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.is_plain_text() {
            self.to_plain_text(res)
        } else if state.targets_html_js() {
            self.to_html_js_component(res)
        } else {
            self.to_markdown(res)
        }
    }
}

impl HtmlJsComponentResult for Quote {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let source = match self.source.as_ref().cloned() {
            Some(s) => Some(s.render(Arc::clone(&res))?),
            None => None,
        };
        let templ = QuoteHtmlTemplate { content: self.content.render(Arc::clone(&res))?,
                                        source,
                                        source_id: self.source_id.as_ref().cloned(),
                                        sizable: self.sizable.clone() };
        templ.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl PlainTextComponentResult for Quote {
    fn to_plain_text(&self,
                     res: crate::lang::runtime::traits::conundrum_input::ArcState)
                     -> ConundrumModalResult<String> {
        todo!()
    }
}

impl MarkdownComponentResult for Quote {
    fn to_markdown(&self,
                   res: crate::lang::runtime::traits::conundrum_input::ArcState)
                   -> ConundrumModalResult<String> {
        // TODO: Actually implement the markdown output.
        todo!()
    }
}

impl ConundrumComponent for Quote {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Quote)
    }

    fn from_props(props: crate::parsers::conundrum::logic::object::object::ConundrumObject,
                  children: Option<Vec<crate::lang::elements::parsed_elements::ParsedElement>>,
                  state: crate::lang::runtime::traits::conundrum_input::ArcState)
                  -> ConundrumModalResult<Self>
        where Self: Sized {
        let content = children.map(Children).ok_or_else(|| {
            ErrMode::Cut(
                ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Missing quote content", "All quote components require `children`, the primary content of the quote. Please see the `Syntax??` docs or the `Quote??` docs for more information."))
            )
        })?;
        let source = Children::from_jsx_props_with_state(&props, "source", Arc::clone(&state)).ok();
        let source_id = ConundrumString::from_jsx_props(&props, "source").ok();
        let sizable = SizablePropsGroup::from_jsx_props(&props, "").unwrap_or_default();
        let markdown_options =
            MarkdownComponentOptions::from_jsx_props(&props,
                                                     CommonComponentPropertyKey::ComponentMarkdownProps.to_string()
                                                                                                       .as_str()).ok();
        Ok(Quote { content,
                   source,
                   source_id,
                   markdown_options,
                   sizable })
    }
}
