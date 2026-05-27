use std::sync::Arc;

use askama::Template;
use serde::Serialize;
use winnow::error::ErrMode;

use crate::{
    lang::{
        lib::ui::ui_types::children::Children,
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            },
            traits::{
                conundrum_input::ArcState, html_js_component_result::HtmlJsComponentResult,
                markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    parsers::markdown::code_block::dictionary::{
        dictionary_entry_html_temp::DictionaryEntryHtmlTemplate,
        dictionary_entry_markdown_templ::DictionaryEntryMarkdownTemplate,
        dictionary_entry_plain_text_templ::DictionaryEntryPlainTextTemplate,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, serde::Deserialize, Clone)]
pub struct DictionaryCodeBlock {
    pub leading_char: char,
    pub title: Children,
    pub content: Children,
}

impl MarkdownComponentResult for DictionaryCodeBlock {
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        let templ = DictionaryEntryMarkdownTemplate { content: self.content.render(Arc::clone(&res))?,
                                                      title: self.title.render(Arc::clone(&res))? };
        templ.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl PlainTextComponentResult for DictionaryCodeBlock {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        let templ = DictionaryEntryPlainTextTemplate { content: self.content.render(Arc::clone(&res))?,
                                                       title: self.title.render(Arc::clone(&res))? };
        templ.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl HtmlJsComponentResult for DictionaryCodeBlock {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let read_state = res.read_arc();
        let note_id = read_state.data.note_id.clone();
        drop(read_state);
        let templ = DictionaryEntryHtmlTemplate { label: self.title.render(Arc::clone(&res))?,
                                                  body: self.content.render(Arc::clone(&res))?,
                                                  note_id };

        templ.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}
