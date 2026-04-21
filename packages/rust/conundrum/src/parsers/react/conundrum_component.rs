use serde::Serialize;
use typeshare::typeshare;

use crate::{
    lang::{
        lib::ui::components::{
            academic::equation_reference::equation_reference_model::EquationReference,
            attention::{admonition::admonition::Admonition, hint::hint::Hint, hl::hl::Highlight, ul::ul::Underline},
            documentation::emoji::emoji_docs_demo::EmojiDocsDemo,
            layout::{
                card::card::Card,
                container::container_model::UtilityContainer,
                grid::grid::ResponsiveGrid,
                tabs::{tabs_group::TabsGroup, tabs_group_tab::Tab},
                toc::table_of_contents::TableOfContents,
            },
        },
        runtime::{
            state::conundrum_error_variant::ConundrumModalResult,
            traits::{
                conundrum_input::ArcState, fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult, markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    parsers::{conundrum::hr_with_children::HrWithChildrenResult, markdown::markdown_extensions::emoji::EmojiResult},
};

#[typeshare]
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum ConundrumComponentType {
    // Layout
    Toc(TableOfContents),
    Container(UtilityContainer),
    Card(Card),
    Tabs(TabsGroup),
    Tab(Tab),
    Grid(ResponsiveGrid),
    Hr(HrWithChildrenResult),
    // Attention
    Admonition(Admonition),
    Hint(Hint),
    Ul(Underline),
    Hl(Highlight),
    Emoji(EmojiResult),
    // Academic
    EqRef(EquationReference),
    // Nested Documentation
    EmojiDocsDemo(EmojiDocsDemo),
}

impl ConundrumComponentType {
    pub fn component_is_block_level(&self) -> bool {
        match self {
            ConundrumComponentType::Card(_) => true,
            ConundrumComponentType::Admonition(_) => true,
            ConundrumComponentType::Hint(_) => true,
            ConundrumComponentType::Ul(_) => false,
            ConundrumComponentType::Hl(_) => false,
            ConundrumComponentType::Container(_) => true,
            ConundrumComponentType::Tabs(_) => true,
            ConundrumComponentType::Tab(_) => true,
            ConundrumComponentType::EqRef(_) => false,
            ConundrumComponentType::Grid(_) => true,
            ConundrumComponentType::Emoji(_) => false,
            ConundrumComponentType::Hr(_) => true,
            ConundrumComponentType::EmojiDocsDemo(_) => true,
            ConundrumComponentType::Toc(_) => true,
        }
    }
}

impl HtmlJsComponentResult for ConundrumComponentType {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        match self {
            ConundrumComponentType::Card(s) => s.to_html_js_component(res),
            ConundrumComponentType::Admonition(s) => s.to_html_js_component(res),
            ConundrumComponentType::Hint(s) => s.to_plain_text(res),
            ConundrumComponentType::Ul(s) => s.to_plain_text(res),
            ConundrumComponentType::Hl(s) => s.to_plain_text(res),
            ConundrumComponentType::Container(s) => s.to_plain_text(res),
            ConundrumComponentType::Tabs(s) => s.to_plain_text(res),
            ConundrumComponentType::Tab(s) => s.to_conundrum_component(res),
            ConundrumComponentType::EqRef(s) => s.to_plain_text(res),
            ConundrumComponentType::Grid(s) => s.to_plain_text(res),
            ConundrumComponentType::Emoji(s) => s.to_plain_text(res),
            ConundrumComponentType::Hr(s) => s.to_plain_text(res),
            ConundrumComponentType::EmojiDocsDemo(s) => s.to_plain_text(res),
            ConundrumComponentType::Toc(s) => s.to_plain_text(res),
        }
    }
}

impl PlainTextComponentResult for ConundrumComponentType {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        match self {
            ConundrumComponentType::Card(s) => s.to_plain_text(res),
            ConundrumComponentType::Admonition(s) => s.to_plain_text(res),
            ConundrumComponentType::Hint(s) => s.to_plain_text(res),
            ConundrumComponentType::Ul(s) => s.to_plain_text(res),
            ConundrumComponentType::Hl(s) => s.to_plain_text(res),
            ConundrumComponentType::Container(s) => s.to_plain_text(res),
            ConundrumComponentType::Tabs(s) => s.to_plain_text(res),
            ConundrumComponentType::Tab(s) => s.to_plain_text(res),
            ConundrumComponentType::EqRef(s) => s.to_plain_text(res),
            ConundrumComponentType::Grid(s) => s.to_plain_text(res),
            ConundrumComponentType::Emoji(s) => s.to_plain_text(res),
            ConundrumComponentType::Hr(s) => s.to_plain_text(res),
            ConundrumComponentType::EmojiDocsDemo(s) => s.to_plain_text(res),
            ConundrumComponentType::Toc(s) => s.to_plain_text(res),
        }
    }
}

impl MarkdownComponentResult for ConundrumComponentType {
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        match self {
            ConundrumComponentType::Card(s) => s.to_markdown(res),
            ConundrumComponentType::Admonition(s) => s.to_markdown(res),
            ConundrumComponentType::Hint(s) => s.to_markdown(res),
            ConundrumComponentType::Ul(s) => s.to_markdown(res),
            ConundrumComponentType::Hl(s) => s.to_markdown(res),
            ConundrumComponentType::Container(s) => s.to_markdown(res),
            ConundrumComponentType::Tabs(s) => s.to_markdown(res),
            ConundrumComponentType::Tab(s) => s.to_markdown(res),
            ConundrumComponentType::EqRef(s) => s.to_markdown(res),
            ConundrumComponentType::Grid(s) => s.to_markdown(res),
            ConundrumComponentType::Emoji(s) => s.to_markdown(res),
            ConundrumComponentType::Hr(s) => s.to_markdown(res),
            ConundrumComponentType::EmojiDocsDemo(s) => s.to_markdown(res),
            ConundrumComponentType::Toc(s) => s.to_markdown(res),
        }
    }
}

impl ConundrumComponentResult for ConundrumComponentType {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        match self {
            ConundrumComponentType::Card(s) => s.to_conundrum_component(res),
            ConundrumComponentType::Admonition(s) => s.to_conundrum_component(res),
            ConundrumComponentType::Hint(s) => s.to_conundrum_component(res),
            ConundrumComponentType::Ul(s) => s.to_conundrum_component(res),
            ConundrumComponentType::Hl(s) => s.to_conundrum_component(res),
            ConundrumComponentType::Container(s) => s.to_conundrum_component(res),
            ConundrumComponentType::Tabs(s) => s.to_conundrum_component(res),
            ConundrumComponentType::Tab(s) => s.to_conundrum_component(res),
            ConundrumComponentType::EqRef(s) => s.to_conundrum_component(res),
            ConundrumComponentType::Grid(s) => s.to_conundrum_component(res),
            ConundrumComponentType::Emoji(s) => s.to_conundrum_component(res),
            ConundrumComponentType::Hr(s) => s.to_conundrum_component(res),
            ConundrumComponentType::EmojiDocsDemo(s) => s.to_conundrum_component(res),
            ConundrumComponentType::Toc(s) => s.to_conundrum_component(res),
        }
    }
}
