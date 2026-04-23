use std::str::FromStr;
use std::sync::Arc;

use parking_lot::Mutex;
use serde::Serialize;
use syntect_assets::assets::HighlightingAssets;
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
            html_js_component_result::HtmlJsComponentResult,
            markdown_component_result::MarkdownComponentResult,
            mdx_component_result::MdxComponentResult,
        },
    },
    parsers::{
        markdown::code_block::{
            general::{
                general_codeblock::GeneralPresentationCodeBlock,
                render_codeblock::{RenderCodeToHtmlReq, render_general_codeblock_to_html},
            },
            supported_languages::SupportedCodeBlockSyntax,
            supported_themes::SupportedCodeBlockTheme,
        },
        parser_trait::ConundrumParser,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct InlineCodeResult {
    pub content: String,
    pub lang: SupportedCodeBlockSyntax,
}

impl InlineCodeResult {
    pub fn get_highlighted_content(&self,
                                   theme: SupportedCodeBlockTheme,
                                   hl: std::sync::Arc<Mutex<HighlightingAssets>>)
                                   -> ConundrumModalResult<String> {
        render_general_codeblock_to_html(RenderCodeToHtmlReq { code: GeneralPresentationCodeBlock { content:
                                                                                                        self.content
                                                                                                            .clone(),
                                                                                                    lang:
                                                                                                        self.lang
                                                                                                            .clone(),
                                                                                                    theme:
                                                                                                        Some(theme),
                                                                                                    inline: true,
                                                                                                    assets: hl } })
    }
}

impl MarkdownComponentResult for InlineCodeResult {
    fn to_markdown(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("`{}`", self.content))
    }
}

impl MdxComponentResult for InlineCodeResult {
    fn to_mdx_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("`{}{{:{}}}`", self.content, self.lang))
    }
}

impl HtmlJsComponentResult for InlineCodeResult {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();

        let highlighted =
            self.get_highlighted_content(state.ui_params.syntax_theme.as_ref().cloned().unwrap_or_default(),
                                         Arc::clone(&state.highlight_assets))?;
        println!("Highlighted: {}", highlighted);
        Ok(format!("<span class=\"cdrm-inline-code inline\">{}</span>", highlighted))
    }
}

pub fn parse_inline_code_block_lang(input: &mut &str) -> ConundrumModalResult<SupportedCodeBlockSyntax> {
    let lang = delimited(literal("{:"), alphanumeric1, '}').parse_next(input)?;
    let l = SupportedCodeBlockSyntax::from_str(lang).unwrap_or(SupportedCodeBlockSyntax::Shell_Unix_Generic);
    Ok(l)
}

pub fn parse_inner_inline_code_block_for_lang(input: &mut &str)
                                              -> ConundrumModalResult<(String, SupportedCodeBlockSyntax)> {
    let (x, lang): (Vec<char>, SupportedCodeBlockSyntax) =
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
                                  lang: SupportedCodeBlockSyntax::Shell_Unix_Generic })
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

        let d = SupportedCodeBlockSyntax::default();
        assert!(matches!(res.lang, d), "Finds the proper language");
        assert!(res.content == "<MyTest myBool myString='Contains a {' />", "Finds the proper content");
    }
    #[test]
    fn parses_inline_code_block_with_lang() {
        let test_content = "`<MyTest myBool myString='Contains a {' />{:tsx}`";
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res = InlineCodeResult::parse_input_string(&mut test_data).expect("Parses inline code block with language tag without throwing an error.");

        assert!(res.lang.to_string() == "tsx".to_string(), "Finds the proper language");
        assert!(res.content == "<MyTest myBool myString='Contains a {' />", "Finds the proper content");
    }
}
