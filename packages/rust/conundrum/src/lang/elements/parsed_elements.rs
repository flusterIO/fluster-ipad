use serde::Serialize;

use crate::{
    lang::{
        lib::ui::ui_types::children::Children,
        runtime::{
            state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState},
            traits::{fluster_component_result::ConundrumComponentResult, mdx_component_result::MdxComponentResult},
        },
    },
    parsers::{
        conundrum::{
            comment::ConundrumCommentResult,
            docs::ParsedInspectionRequest,
            hr_with_children::HrWithChildrenResult,
            inline_citation::ParsedCitation,
            logic::{
                bool::boolean::ConundrumBoolean, number::conundrum_number::ConundrumNumber, token::ConundrumLogicToken,
            },
            note_link::ParsedOutgoingNoteLink,
            tag::ParsedTag,
        },
        javascript::parsed_javascript_elements::ParsedJavascriptElement,
        markdown::{
            block_math::BlockMathResult, block_quote::BlockQuoteResult,
            bold_and_italic_text::MarkdownBoldAndItalicTextResult, bold_text::MarkdownBoldTextResult,
            code_block::ParsedCodeBlock, heading::MarkdownHeadingResult, inline_code::InlineCodeResult,
            inline_math::InlineMathResult, italic_text::MarkdownItalicTextResult,
            markdown_extensions::emoji::EmojiResult, markdown_link::MarkdownLinkResult,
            paragraph::MarkdownParagraphResult,
        },
        react::{
            react_component_self_closing::ReactComponentSelfClosingResult,
            react_component_with_children::ReactComponentWithChildrenResult,
        },
    },
};

impl MdxComponentResult for String {
    fn to_mdx_component(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
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
    BoldText(MarkdownBoldTextResult),
    ItalicText(MarkdownItalicTextResult),
    BoldAndItalicText(MarkdownBoldAndItalicTextResult),
    ParsedCodeBlock(ParsedCodeBlock),
    InlineCode(InlineCodeResult),
    MarkdownLink(MarkdownLinkResult),
    MarkdownParagraph(MarkdownParagraphResult),
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

impl MdxComponentResult for ParsedElement {
    fn to_mdx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        match self {
            ParsedElement::ParsedInspectionRequest(req) => req.to_conundrum_component(res),
            ParsedElement::ParsedCodeBlock(block) => block.to_conundrum_component(res),
            ParsedElement::ParsedCitation(cite) => cite.to_conundrum_component(res),
            ParsedElement::ParsedOutgoingNoteLink(l) => l.to_conundrum_component(res),
            ParsedElement::Tag(tag) => tag.to_conundrum_component(res),
            ParsedElement::Text(s) => Ok(s.clone()),
            ParsedElement::Heading(heading) => heading.to_conundrum_component(res),
            ParsedElement::BlockMath(math) => math.to_conundrum_component(res),
            ParsedElement::InlineMath(math) => math.to_conundrum_component(res),
            ParsedElement::BlockQuote(quote) => quote.to_conundrum_component(res),
            ParsedElement::BoldText(t) => t.to_conundrum_component(res),
            ParsedElement::ItalicText(t) => t.to_conundrum_component(res),
            ParsedElement::BoldAndItalicText(t) => t.to_conundrum_component(res),
            ParsedElement::MarkdownLink(l) => l.to_conundrum_component(res),
            ParsedElement::MarkdownParagraph(p) => p.to_conundrum_component(res),
            ParsedElement::HrWithChildren(c) => c.to_conundrum_component(res),
            ParsedElement::Comment(c) => c.to_conundrum_component(res),
            ParsedElement::ReactComponentSelfClosing(c) => c.component.to_conundrum_component(res),
            ParsedElement::ReactComponentWithChildren(c) => c.component.to_conundrum_component(res),
            ParsedElement::Emoji(e) => e.to_conundrum_component(res),
            ParsedElement::InlineCode(m) => m.to_conundrum_component(res),
            ParsedElement::Children(c) => c.render(res),
            ParsedElement::Javascript(js) => js.to_conundrum_component(res),
            ParsedElement::Logic(l) => l.to_conundrum_component(res),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_md() {
        // let res =
        // assert_eq!(result, 4);
    }
}
