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
                conundrum_input::ArcState, fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult, markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult, plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    parsers::{
        conundrum::{
            comment::ConundrumCommentResult,
            docs::docs_model::ParsedInspectionRequest,
            hr_with_children::hr_with_children_model::HrWithChildrenResult,
            inline_citation::ParsedCitation,
            logic::{string::conundrum_string::ConundrumString, token::ConundrumLogicToken},
            note_link::note_link_model::ParsedOutgoingNoteLink,
            tag::ParsedTag,
        },
        javascript::parsed_javascript_elements::ParsedJavascriptElement,
        markdown::{
            block_math::BlockMathResult,
            block_quote::block_quote_model::BlockQuoteResult,
            bold_and_italic_text::MarkdownBoldAndItalicTextResult,
            bold_text::MarkdownBoldTextResult,
            code_block::code_block_model::ParsedCodeBlock,
            heading::heading_model::MarkdownHeadingResult,
            hr::MarkdownHorizontalRule,
            inline_code::InlineCodeResult,
            inline_math::InlineMathResult,
            italic_text::MarkdownItalicTextResult,
            lists::{
                ordered::ordered_list_model::OrderedListModel, unordered::unordered_list_model::UnorderedListModel,
            },
            markdown_extensions::emoji::emoji_model::EmojiResult,
            markdown_link::MarkdownLinkResult,
            paragraph::paragraph_model::MarkdownParagraphResult,
            table::markdown_table_model::MarkdownTable,
        },
        react::{
            react_component_self_closing::ReactComponentSelfClosingResult,
            react_component_with_children::ReactComponentWithChildrenResult,
        },
    },
};

impl MdxComponentResult for String {
    fn to_mdx_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(self.clone())
    }
}

// This enum acts as a router for our different syntaxes
// As a current limitation, a parser cannot have children as
// `Vec<ParsedElement>` or reference `ParsedElement` in any other way _and_
// export it's type via typeshare.
#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum ParsedElement {
    // Markdown
    Heading(MarkdownHeadingResult),
    BlockQuote(BlockQuoteResult),
    BlockMath(BlockMathResult),
    InlineMath(InlineMathResult),
    Text(String),
    Hr(MarkdownHorizontalRule),
    BoldText(MarkdownBoldTextResult),
    ItalicText(MarkdownItalicTextResult),
    BoldAndItalicText(MarkdownBoldAndItalicTextResult),
    ParsedCodeBlock(ParsedCodeBlock),
    InlineCode(InlineCodeResult),
    MarkdownLink(MarkdownLinkResult),
    MarkdownParagraph(MarkdownParagraphResult),
    UnorderedList(UnorderedListModel),
    OrderedList(OrderedListModel),
    Table(MarkdownTable),
    // Markdown Extensions
    Emoji(EmojiResult),
    // React
    ReactComponentSelfClosing(ReactComponentSelfClosingResult),
    ReactComponentWithChildren(ReactComponentWithChildrenResult),
    // Fluster Specific
    ParsedCitation(ParsedCitation),
    ParsedOutgoingNoteLink(ParsedOutgoingNoteLink),
    Tag(ParsedTag),
    ParsedInspectionRequest(ParsedInspectionRequest),
    HrWithChildren(HrWithChildrenResult),
    Comment(ConundrumCommentResult),
    Children(Children),
    Javascript(ParsedJavascriptElement),
    Logic(ConundrumLogicToken),
}

impl HtmlJsComponentResult for ParsedElement {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        match self {
            ParsedElement::ParsedInspectionRequest(req) => req.to_html_js_component(res),
            ParsedElement::ParsedCodeBlock(block) => block.to_html_js_component(res),
            ParsedElement::ParsedCitation(cite) => cite.to_html_js_component(res),
            ParsedElement::ParsedOutgoingNoteLink(l) => l.to_html_js_component(res),
            ParsedElement::Tag(tag) => tag.to_html_js_component(res),
            ParsedElement::Text(s) => Ok(s.clone()),
            ParsedElement::Heading(heading) => heading.to_html_js_component(res),
            ParsedElement::BlockMath(math) => math.to_html_js_component(res),
            ParsedElement::InlineMath(math) => math.to_html_js_component(res),
            ParsedElement::BlockQuote(quote) => quote.to_html_js_component(res),
            ParsedElement::BoldText(t) => t.to_html_js_component(res),
            ParsedElement::ItalicText(t) => t.to_html_js_component(res),
            ParsedElement::BoldAndItalicText(t) => t.to_html_js_component(res),
            ParsedElement::MarkdownLink(l) => l.to_html_js_component(res),
            ParsedElement::MarkdownParagraph(p) => p.to_html_js_component(res),
            ParsedElement::Hr(l) => l.to_html_js_component(res),
            ParsedElement::HrWithChildren(c) => c.to_html_js_component(res),
            ParsedElement::Comment(c) => c.to_conundrum_component(res),
            ParsedElement::ReactComponentSelfClosing(c) => c.component.to_html_js_component(res),
            ParsedElement::ReactComponentWithChildren(c) => c.component.to_html_js_component(res),
            ParsedElement::Emoji(e) => e.to_html_js_component(res),
            ParsedElement::InlineCode(m) => m.to_html_js_component(res),
            ParsedElement::Children(c) => c.render(res),
            ParsedElement::Javascript(js) => js.to_conundrum_component(res),
            ParsedElement::Logic(l) => l.to_conundrum_component(res),
            ParsedElement::UnorderedList(l) => l.to_html_js_component(res),
            ParsedElement::OrderedList(l) => l.to_html_js_component(res),
            ParsedElement::Table(t) => t.to_html_js_component(res),
        }
    }
}

impl MdxComponentResult for ParsedElement {
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        match self {
            ParsedElement::ParsedInspectionRequest(req) => req.to_conundrum_component(res),
            ParsedElement::ParsedCodeBlock(block) => block.to_conundrum_component(res),
            ParsedElement::ParsedCitation(cite) => cite.to_conundrum_component(res),
            ParsedElement::ParsedOutgoingNoteLink(l) => l.to_conundrum_component(res),
            ParsedElement::Tag(tag) => tag.to_conundrum_component(res),
            ParsedElement::Text(s) => Ok(s.clone()),
            ParsedElement::Heading(heading) => heading.to_mdx_component(res),
            ParsedElement::BlockMath(math) => math.to_conundrum_component(res),
            ParsedElement::InlineMath(math) => math.to_mdx_component(res),
            ParsedElement::BlockQuote(quote) => quote.to_conundrum_component(res),
            ParsedElement::BoldText(t) => t.to_mdx_component(res),
            ParsedElement::ItalicText(t) => t.to_mdx_component(res),
            ParsedElement::BoldAndItalicText(t) => t.to_conundrum_component(res),
            ParsedElement::MarkdownLink(l) => l.to_mdx_component(res),
            ParsedElement::MarkdownParagraph(p) => p.to_mdx_component(res),
            ParsedElement::HrWithChildren(c) => c.to_conundrum_component(res),
            ParsedElement::Hr(c) => c.to_conundrum_component(res),
            ParsedElement::Comment(c) => c.to_conundrum_component(res),
            ParsedElement::ReactComponentSelfClosing(c) => c.component.to_conundrum_component(res),
            ParsedElement::ReactComponentWithChildren(c) => c.component.to_conundrum_component(res),
            ParsedElement::Emoji(e) => e.to_conundrum_component(res),
            ParsedElement::InlineCode(m) => m.to_mdx_component(res),
            ParsedElement::Children(c) => c.render(res),
            ParsedElement::Javascript(js) => js.to_conundrum_component(res),
            ParsedElement::Logic(l) => l.to_conundrum_component(res),
            ParsedElement::UnorderedList(l) => l.to_conundrum_component(res),
            ParsedElement::OrderedList(l) => l.to_conundrum_component(res),
            ParsedElement::Table(t) => t.to_conundrum_component(res),
        }
    }
}

impl PlainTextComponentResult for ParsedElement {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        match self {
            ParsedElement::ParsedInspectionRequest(req) => req.to_plain_text(res),
            ParsedElement::ParsedCodeBlock(block) => block.to_plain_text(res),
            ParsedElement::ParsedCitation(cite) => cite.to_plain_text(res),
            ParsedElement::ParsedOutgoingNoteLink(l) => l.to_plain_text(res),
            ParsedElement::Tag(tag) => tag.to_plain_text(res),
            ParsedElement::Text(s) => Ok(s.clone()),
            ParsedElement::Heading(heading) => heading.to_plain_text(res),
            ParsedElement::BlockMath(math) => math.to_plain_text(res),
            ParsedElement::InlineMath(math) => math.to_plain_text(res),
            ParsedElement::BlockQuote(quote) => quote.to_plain_text(res),
            ParsedElement::BoldText(t) => t.to_plain_text(res),
            ParsedElement::ItalicText(t) => t.to_plain_text(res),
            ParsedElement::BoldAndItalicText(t) => t.to_plain_text(res),
            ParsedElement::MarkdownLink(l) => l.to_plain_text(res),
            ParsedElement::MarkdownParagraph(p) => p.to_plain_text(res),
            ParsedElement::HrWithChildren(c) => c.to_plain_text(res),
            ParsedElement::Hr(c) => c.to_plain_text(res),
            ParsedElement::Comment(c) => c.to_plain_text(res),
            ParsedElement::ReactComponentSelfClosing(c) => c.component.to_plain_text(res),
            ParsedElement::ReactComponentWithChildren(c) => c.component.to_plain_text(res),
            ParsedElement::Emoji(e) => e.to_plain_text(res),
            ParsedElement::InlineCode(m) => m.to_markdown(res),
            ParsedElement::Children(c) => c.render(res),
            ParsedElement::Javascript(js) => js.to_conundrum_component(res),
            ParsedElement::Logic(l) => l.to_conundrum_component(res),
            ParsedElement::UnorderedList(l) => l.to_conundrum_component(res),
            ParsedElement::OrderedList(l) => l.to_plain_text(res),
            ParsedElement::Table(t) => t.to_plain_text(res),
        }
    }
}

impl ParsedElement {
    pub fn get_string(&self) -> ConundrumModalResult<ConundrumString> {
        match self {
            ParsedElement::Logic(l) => match l {
                ConundrumLogicToken::String(s) => Some(s.clone()),
                _ => None
            },
            _ => None
        }.ok_or_else(|| {
            ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Fail to find string")))
        })
    }

    /// Currently this logic is in like 3 places, but this is the final source
    /// of truth for what's block level and what's not. I'll replace all of
    /// those when I have time.
    ///
    /// This returns false if the element can exist in a paragraph, and true if
    /// it can't.
    pub fn is_block_level(&self) -> bool {
        match self {
            ParsedElement::ParsedInspectionRequest(_) => true,
            ParsedElement::ParsedCodeBlock(_) => true,
            ParsedElement::ParsedCitation(_) => false,
            ParsedElement::ParsedOutgoingNoteLink(_) => false,
            ParsedElement::Tag(_) => false,
            // The text element is exempt from the 'at_line_start' requirement as it's filtered
            // out there.
            ParsedElement::Text(_) => false,
            ParsedElement::Heading(_) => true,
            ParsedElement::BlockMath(_) => true,
            ParsedElement::InlineMath(_) => false,
            ParsedElement::BlockQuote(_) => true,
            ParsedElement::BoldText(_) => false,
            ParsedElement::ItalicText(_) => false,
            ParsedElement::BoldAndItalicText(_) => false,
            ParsedElement::MarkdownLink(_) => false,
            ParsedElement::MarkdownParagraph(_) => true,
            ParsedElement::HrWithChildren(_) => true,
            ParsedElement::Hr(_) => true,
            ParsedElement::Comment(_) => true,
            ParsedElement::ReactComponentSelfClosing(c) => c.component.component_is_block_level(),
            ParsedElement::ReactComponentWithChildren(c) => c.component.component_is_block_level(),
            ParsedElement::Emoji(_) => false,
            ParsedElement::InlineCode(_) => false,
            ParsedElement::Children(c) => c.0.iter().any(|child| child.is_block_level()),
            ParsedElement::Javascript(_s) => false,
            ParsedElement::Logic(_) => false,
            ParsedElement::UnorderedList(_) => true,
            ParsedElement::OrderedList(_) => true,
            ParsedElement::Table(_) => true,
        }
    }
}
