use askama::Template;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;
use std::sync::Arc;
use tabled::{Table, settings::Style};
use typeshare::typeshare;
use winnow::error::ErrMode;

use crate::{
    lang::{
        lib::ui::components::{
            attention::emoji::currently_supported_emoji_names::CURRENTLY_SUPPORTED_EMOJI_NAMES,
            component_trait::ConundrumComponent,
            documentation::emoji::{emoji_data::EmojiData, emoji_docs_html_templ::EmojiDocsHtmlTemplate},
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::ConundrumCompileTarget,
            },
            traits::{
                conundrum_input::ArcState, conundrum_template::ConundrumTemplateRepresentable,
                fluster_component_result::ConundrumComponentResult, html_js_component_result::HtmlJsComponentResult,
                jsx_component_result::JsxComponentResult, markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, documentation_component_name::DocumentationComponentName,
    },
};

/// This allows you to explore the twemoji set of ~4800 emojis, but don't embed
/// this directly. This is automatically embedded in the docs container when you
/// use the `Emoji??` documentation flag.
#[typeshare]
#[derive(Debug, Serialize, serde::Deserialize, Clone)]
pub struct EmojiDocsDemo {}

impl EmojiDocsDemo {
    pub fn get_emoji_data(&self) -> Vec<EmojiData> {
        CURRENTLY_SUPPORTED_EMOJI_NAMES.par_iter()
                                       .filter_map(|name| {
                                           twemoji_assets::svg::SvgTwemojiAsset::from_name(name).map(|item| {
                    EmojiData { name: name.to_string(), svg: item.data.0.to_string() }
                })
                                       })
                                       .collect::<Vec<EmojiData>>()
    }
}

impl MarkdownComponentResult for EmojiDocsDemo {
    fn to_markdown(&self,
                   _: ArcState)
                   -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        let emojis =
            CURRENTLY_SUPPORTED_EMOJI_NAMES.par_iter()
                                           .filter_map(|name| {
                                               twemoji_assets::svg::SvgTwemojiAsset::from_name(name).map(|item| {
    EmojiData { name: name.to_string(), svg: item.data.0.to_string() }
})
                                           })
                                           .collect::<Vec<EmojiData>>();

        let mut table = Table::new(emojis);
        let style = Style::markdown();
        table.with(style);

        Ok(table.to_string())
    }
}

impl PlainTextComponentResult for EmojiDocsDemo {
    fn to_plain_text(&self,
                     _: ArcState)
                     -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        let mut s = String::from("Emoji names:\n");
        for item in CURRENTLY_SUPPORTED_EMOJI_NAMES.iter() {
            s += format!("- \"{}\"\n", item).as_str();
        }
        Ok(s)
    }
}

impl ConundrumTemplateRepresentable<EmojiDocsHtmlTemplate> for EmojiDocsDemo {
    fn to_template(
        &self,
        state: ArcState)
        -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<EmojiDocsHtmlTemplate> {
        let items = self.get_emoji_data().iter().take(50).cloned().collect();
        Ok(EmojiDocsHtmlTemplate { items })
    }
}

impl HtmlJsComponentResult for EmojiDocsDemo {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.to_template(Arc::clone(&res))?.render().map_err(|e| {
                  eprintln!("Error: {:#?}", e);
                  ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
              })
    }
}

impl JsxComponentResult for EmojiDocsDemo {
    fn to_jsx_component(&self,
                        _: ArcState)
                        -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        Ok(format!("<{} />", DocumentationComponentName::EmojiDocumentationDemo))
    }
}

impl ConundrumComponentResult for EmojiDocsDemo {
    fn to_conundrum_component(&self,
                              res: ArcState)
                              -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String>
    {
        let state = res.read_arc();
        if state.compile_target == ConundrumCompileTarget::PlainText {
            drop(state);
            self.to_plain_text(res)
        } else if state.targets_markdown() {
            drop(state);
            self.to_markdown(res)
        } else {
            drop(state);
            self.to_jsx_component(res)
        }
    }
}

impl ConundrumComponent for EmojiDocsDemo {
    fn from_props(_: crate::parsers::conundrum::logic::object::object::ConundrumObject,
                  _: Option<Vec<crate::lang::elements::parsed_elements::ParsedElement>>,
                  _: ArcState)
                  -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<Self>
        where Self: Sized {
        Ok(EmojiDocsDemo {})
    }

    fn get_component_id() -> AnyComponentName {
        AnyComponentName::Docs(DocumentationComponentName::EmojiDocumentationDemo)
    }
}
