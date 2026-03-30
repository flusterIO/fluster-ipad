use serde::Serialize;
use winnow::{
    ModalResult, Parser,
    ascii::alphanumeric1,
    combinator::{alt, delimited, opt, repeat_till},
    stream::{AsChar, Stream},
    token::{any, literal, take_till},
};

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::{
        markdown::{code_block, subtypes::code_block_language::CodeBlockLanguage},
        parser_trait::ConundrumParser,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize)]
pub struct InlineCodeResult {
    pub content: String,
    pub lang: CodeBlockLanguage,
}

pub fn parse_inline_code_block_lang(input: &mut &str) -> ModalResult<CodeBlockLanguage> {
    let lang = delimited(literal("{:"), alphanumeric1, '}').parse_next(input)?;
    Ok(CodeBlockLanguage::UserProvided(lang.to_string()))
}

pub fn parse_inner_inline_code_block_for_lang(input: &mut &str) -> ModalResult<(String, CodeBlockLanguage)> {
    let (x, lang): (Vec<char>, CodeBlockLanguage) =
        repeat_till(1.., any, parse_inline_code_block_lang).parse_next(input)?;
    let content = String::from_iter(x);
    Ok((content, lang))
}

fn inline_code_block_content(input: &mut ConundrumInput) -> ModalResult<String> {
    let res = take_till(1.., |c| c == '`' || AsChar::is_newline(c)).parse_next(input)?;
    Ok(res.to_string())
}

impl ConundrumParser<InlineCodeResult> for InlineCodeResult {
    fn parse_input_string(input: &mut crate::lang::runtime::traits::conundrum_input::ConundrumInput)
                          -> winnow::ModalResult<InlineCodeResult> {
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
    fn parses_inline_code_block_with_lang() {
        let test_content = "`<MyTest myBool myString='Contains a {' />{:tsx}`";
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res = InlineCodeResult::parse_input_string(&mut test_data).expect("Parses inline code block with language tag without throwing an error.");

        println!("Res: {:#?}", res);

        assert!(match res.lang {
                    CodeBlockLanguage::UserProvided(s) => s == "tsx",
                    _ => false,
                },
                "Finds the proper language");
        assert!(res.content == "<MyTest myBool myString='Contains a {' />", "Finds the proper content");
        // assert_eq!(result, 4);
    }
}
