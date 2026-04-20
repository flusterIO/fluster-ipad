use serde::Serialize;
use winnow::{
    Parser,
    ascii::alphanumeric1,
    combinator::{delimited, opt, repeat_till},
    stream::AsChar,
    token::{any, literal, take_till},
};

use crate::{
    lang::runtime::{
        state::conundrum_error_variant::ConundrumModalResult,
        traits::{
            conundrum_input::{ArcState, ConundrumInput},
            markdown_component_result::MarkdownComponentResult,
            mdx_component_result::MdxComponentResult,
        },
    },
    parsers::{markdown::subtypes::code_block_language::CodeBlockLanguage, parser_trait::ConundrumParser},
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct InlineCodeResult {
    pub content: String,
    pub lang: CodeBlockLanguage,
}

impl MarkdownComponentResult for InlineCodeResult {
    fn to_markdown(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("`{}`", self.content))
    }
}

impl MdxComponentResult for InlineCodeResult {
    fn to_mdx_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        match &self.lang {
            CodeBlockLanguage::DefaultLanguage => Ok(format!("`{}`", self.content)),
            CodeBlockLanguage::UserProvided(s) => Ok(format!("`{}{{:{}}}`", self.content, s)),
        }
    }
}

pub fn parse_inline_code_block_lang(input: &mut &str) -> ConundrumModalResult<CodeBlockLanguage> {
    let lang = delimited(literal("{:"), alphanumeric1, '}').parse_next(input)?;
    Ok(CodeBlockLanguage::UserProvided(lang.to_string()))
}

pub fn parse_inner_inline_code_block_for_lang(input: &mut &str) -> ConundrumModalResult<(String, CodeBlockLanguage)> {
    let (x, lang): (Vec<char>, CodeBlockLanguage) =
        repeat_till(1.., any, parse_inline_code_block_lang).parse_next(input)?;
    let content = String::from_iter(x);
    Ok((content, lang))
}

fn inline_code_block_content(input: &mut ConundrumInput) -> ConundrumModalResult<String> {
    let res = take_till(1.., |c| c == '`' || AsChar::is_newline(c)).parse_next(input)?;
    Ok(res.to_string())
}

impl ConundrumParser<InlineCodeResult> for InlineCodeResult {
    fn parse_input_string(input: &mut crate::lang::runtime::traits::conundrum_input::ConundrumInput)
                          -> ConundrumModalResult<InlineCodeResult> {
        let code_block = delimited('`', inline_code_block_content, '`').parse_next(input)?;
        if let Some((code_block_content, lang)) =
            opt(parse_inner_inline_code_block_for_lang).parse_next(&mut code_block.as_ref())?
        {
            Ok(InlineCodeResult { content: code_block_content,
                                  lang })
        } else {
            Ok(InlineCodeResult { content: code_block,
                                  lang: CodeBlockLanguage::DefaultLanguage })
        }
    }

    fn matches_first_char(char: char) -> bool {
        char == '`'
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_inline_code_block_with_no_lang() {
        let test_content = "`<MyTest myBool myString='Contains a {' />`";
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res = InlineCodeResult::parse_input_string(&mut test_data).expect("Parses inline code block with language tag without throwing an error.");

        assert!(matches!(res.lang, CodeBlockLanguage::DefaultLanguage), "Finds the proper language");
        assert!(res.content == "<MyTest myBool myString='Contains a {' />", "Finds the proper content");
    }
    #[test]
    fn parses_inline_code_block_with_lang() {
        let test_content = "`<MyTest myBool myString='Contains a {' />{:tsx}`";
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res = InlineCodeResult::parse_input_string(&mut test_data).expect("Parses inline code block with language tag without throwing an error.");

        assert!(match res.lang {
                    CodeBlockLanguage::UserProvided(s) => s == "tsx",
                    _ => false,
                },
                "Finds the proper language");
        assert!(res.content == "<MyTest myBool myString='Contains a {' />", "Finds the proper content");
    }
}
