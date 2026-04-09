use serde::Serialize;
use winnow::{
    Parser,
    combinator::{alt, delimited},
    token::take_while,
};

use crate::{
    lang::{
        lib::{shared::traits::from_with_state::FromWithState, ui::ui_types::children::Children},
        runtime::{
            state::{
                conundrum_error_variant::ConundrumModalResult,
                parse_state::{ConundrumModifier, ParseState},
            },
            traits::{
                conundrum_input::ConundrumInput, fluster_component_result::ConundrumComponentResult,
                markdown_component_result::MarkdownComponentResult, mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    parsers::parser_trait::ConundrumParser,
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct MarkdownItalicTextResult {
    pub children: Children,
}

impl MarkdownComponentResult for MarkdownItalicTextResult {
    fn to_markdown(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(format!("_{}_", self.children.render(res)?))
    }
}

impl PlainTextComponentResult for MarkdownItalicTextResult {
    fn to_plain_text(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl MdxComponentResult for MarkdownItalicTextResult {
    fn to_mdx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(format!("<span className=\"italic\">{}</span>", self.children.render(res)?))
    }
}

impl ConundrumComponentResult for MarkdownItalicTextResult {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if res.contains_one_of_modifiers(vec![ConundrumModifier::ForcePlainText, ConundrumModifier::ForSearchInput]) {
            self.to_plain_text(res)
        } else if res.contains_one_of_modifiers(vec![ConundrumModifier::PreferMarkdownSyntax,
                                                     ConundrumModifier::PreferInlineMarkdownSyntax,])
        {
            self.to_markdown(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl ConundrumParser<MarkdownItalicTextResult> for MarkdownItalicTextResult {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<MarkdownItalicTextResult> {
        let content = alt((delimited('*', take_while(1.., |c| c != '\n' && c != '*'), '*'),
                           delimited('_', take_while(1.., |c| c != '\n' && c != '_'), '_'))).parse_next(input)?;

        let mut state = input.state.borrow_mut();

        let children = Children::from_with_state(content, &mut state)?;

        Ok(MarkdownItalicTextResult { children })
    }

    fn matches_first_char(char: char) -> bool {
        char == '$'
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn markdown_italic_text_asterisk() {
        let test_input = "*My italic text*";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownItalicTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        let mut state = wrapped.state.borrow_mut();

        assert!(res.children.render(&mut state).is_ok_and(|s| s == "My italic text"),
                "Finds the proper text in the markdown italic text with asterisks.");
    }

    #[test]
    fn markdown_italic_text_underscores() {
        let test_input = "_My italic text_";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownItalicTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");
        let mut state = wrapped.state.borrow_mut();

        assert!(res.children.render(&mut state).is_ok_and(|s| s == "My italic text"),
                "Finds the proper text in the markdown italic text with asterisks.");
    }
}
