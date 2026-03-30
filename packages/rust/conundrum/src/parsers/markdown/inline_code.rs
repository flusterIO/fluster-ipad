use serde::Serialize;
use winnow::{
    ModalResult, Parser,
    ascii::alphanumeric1,
    combinator::{alt, delimited, repeat_till},
    stream::{AsChar, Stream},
    token::{any, literal, take_till},
};

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::{markdown::{code_block, subtypes::code_block_language::CodeBlockLanguage}, parser_trait::ConundrumParser},
};

#[typeshare::typeshare]
#[derive(Debug, Serialize)]
pub struct InlineCodeResult {
    pub content: String,
    pub lang: CodeBlockLanguage,
}

pub fn parse_inline_code_block_lang(input: &mut &str) -> ModalResult<CodeBlockLanguage> {
    let start = input.checkpoint();
    // let code_content = 
    literal("{:").void().parse_next(input).inspect_err(|_| {
                                               input.reset(&start);
                                           })?;
    let lang = alphanumeric1.parse_next(input).inspect_err(|_| {
                                                   input.reset(&start);
                                               })?;
    '}'.void().verify(|_| input.is_empty()).parse_next(input).inspect_err(|_| {
                                                                  input.reset(&start);
                                                              })?;
    Ok(CodeBlockLanguage::UserProvided(lang.to_string()))
}

fn inline_code_block_content(input: &mut ConundrumInput) -> ModalResult<String> {
    let res = take_till(1.., |c| c == '`' || c == '{' || AsChar::is_newline(c)).parse_next(input)?;
    Ok(res.to_string())
}

fn inline_code_block_no_lang_tag(input: &mut ConundrumInput) -> ModalResult<InlineCodeResult> {
    let res = delimited('`', inline_code_block_content, '`').parse_next(input)?;
    Ok(InlineCodeResult { content: res.to_string(),
                          lang: CodeBlockLanguage::DefaultLanguage })
}

impl ConundrumParser<InlineCodeResult> for InlineCodeResult {
    fn parse_input_string(input: &mut crate::lang::runtime::traits::conundrum_input::ConundrumInput)
                          -> winnow::ModalResult<InlineCodeResult> {
        let mut code_block = delimited('`', inline_code_block_content, '`').parse_next(input)?;
        // let lang = repeat_till(0.., any, terminator)
        let lang = parse_inline_code_block_lang(&mut code_block.as_str()).unwrap_or_else(|_| CodeBlockLanguage::DefaultLanguage);
        Ok(InlineCodeResult { content: code_block, lang })
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
