use serde::Serialize;

use crate::{
    lang::runtime::{
        state::conundrum_error_variant::ConundrumModalResult,
        traits::{
            conundrum_input::ArcState, html_js_component_result::HtmlJsComponentResult,
            markdown_component_result::MarkdownComponentResult, plain_text_component_result::PlainTextComponentResult,
        },
    },
    parsers::markdown::code_block::{
        code_block_model::GeneralCodeBlock, dictionary::dictionary_code_block::DictionaryCodeBlock,
        mermaid::mermaid_code_block::MermaidCodeBlock,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, serde::Deserialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum ParsedCodeBlockVariant {
    General(GeneralCodeBlock),
    Dictionary(DictionaryCodeBlock),
    AI(GeneralCodeBlock),
    // Mermaid(MermaidCodeBlock),
}

impl MarkdownComponentResult for ParsedCodeBlockVariant {
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        match self {
            ParsedCodeBlockVariant::AI(a) => a.to_markdown(res),
            ParsedCodeBlockVariant::Dictionary(d) => d.to_markdown(res),
            ParsedCodeBlockVariant::General(g) => g.to_markdown(res),
            // ParsedCodeBlockVariant::Mermaid(g) => g.to_markdown(res),
        }
    }
}

impl PlainTextComponentResult for ParsedCodeBlockVariant {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        match self {
            ParsedCodeBlockVariant::AI(a) => a.to_plain_text(res),
            ParsedCodeBlockVariant::Dictionary(d) => d.to_plain_text(res),
            ParsedCodeBlockVariant::General(g) => g.to_plain_text(res),
            // ParsedCodeBlockVariant::Mermaid(g) => g.to_plain_text(res),
        }
    }
}

impl HtmlJsComponentResult for ParsedCodeBlockVariant {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        match self {
            ParsedCodeBlockVariant::AI(a) => a.to_html_js_component(res),
            ParsedCodeBlockVariant::Dictionary(d) => d.to_html_js_component(res),
            ParsedCodeBlockVariant::General(g) => g.to_html_js_component(res),
            // ParsedCodeBlockVariant::Mermaid(g) => g.to_html_js_component(res),
        }
    }
}
