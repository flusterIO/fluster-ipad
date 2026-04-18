use std::str::FromStr;

use askama::Template;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use winnow::{
    Parser,
    ascii::{line_ending, space0, space1},
    combinator::{self},
    error::ErrMode,
    stream::{AsChar, Stream},
    token::{literal, take_till, take_until, take_while},
};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::{ConundrumModifier, ParseState},
                ui_params::UIParams,
            },
            traits::{
                conundrum_input::{ConundrumInput, get_conundrum_input},
                fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult,
                markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::{
        general::component_constants::parser_ids::ParserId,
        output_components::{
            ai_parsing_request_phase_1::get_ai_parsing_request_phase_1_content::get_ai_parsing_request_phase_1_content,
            dictionary_entry::get_dictionary_entry_content::get_dictionary_content,
        },
    },
    parsers::{
        conundrum::logic::{object::object::ConundrumObject, token::ConundrumLogicToken},
        markdown::code_block::{
            code_block_html_template::CodeBlockHTMLTemplate,
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

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct ParsedCodeBlock {
    pub language: SupportedCodeBlockSyntax,
    pub meta_data: Option<String>,
    pub depth: u8,
    pub content: String,
    pub full_match: String,
}

impl ParsedCodeBlock {
    pub fn get_meta_data(&self) -> Option<ConundrumObject> {
        if let Some(meta) = &self.meta_data {
            let input = &mut get_conundrum_input(meta.as_str(), vec![], UIParams::default());
            ConundrumObject::from_single_line_property_string_parser(input).ok()
        } else {
            None
        }
    }

    pub fn get_title(&self) -> Option<String> {
        if let Some(x) = self.get_meta_data() {
            if let Some(title_em) = x.data.get("title") {
                if let Some(title_string) = match title_em.value() {
                    #[allow(clippy::collapsible_match)]
                    ParsedElement::Logic(l) => match l {
                        ConundrumLogicToken::String(s) => Some(s),
                        _ => None,
                    },
                    _ => None,
                } {
                    return Some(title_string.0.clone());
                }
            }
        }
        None
    }

    pub fn get_highlighted_content(&self, theme: SupportedCodeBlockTheme) -> ConundrumModalResult<String> {
        render_general_codeblock_to_html(RenderCodeToHtmlReq { code: GeneralPresentationCodeBlock { content:
                                                                                                        self.content
                                                                                                            .clone(),
                                                                                                    lang:
                                                                                                        self.language
                                                                                                            .clone(),
                                                                                                    theme:
                                                                                                        Some(theme),
                                                                                                    inline: false } })
    }
}

impl PlainTextComponentResult for ParsedCodeBlock {
    fn to_plain_text(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.to_markdown(res)
    }
}

impl MarkdownComponentResult for ParsedCodeBlock {
    fn to_markdown(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
        let mut tick_string = String::from("");
        for _ in 0..self.depth {
            tick_string += "`";
        }
        Ok(format!(
                   "{}{}
{}
{}",
                   tick_string,
                   self.language.markdown_representation(),
                   self.content,
                   tick_string
        ))
    }
}

impl HtmlJsComponentResult for ParsedCodeBlock {
    fn to_html_js_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        let id = res.dom.new_id();
        let code_string = self.get_highlighted_content(res.ui_params.syntax_theme.clone().unwrap_or_default())?;
        println!("Code: {:#?}", code_string);
        let template =
            CodeBlockHTMLTemplate::new(code_string, self.get_title(), id, &self.language, &res.ui_params.dark_mode);
        template.render().map_err(|e| {
                             eprintln!("Error: {:#?}", e);
                             ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("")))
                         })
    }
}

impl ConundrumComponentResult for ParsedCodeBlock {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        match self.language {
            SupportedCodeBlockSyntax::Dictionary => {
                if res.should_ignore_parser(&ParserId::Dictionary) {
                    return Ok(self.full_match.clone());
                }

                // Extract the metadata or provide a fallback
                Ok(get_dictionary_content(self, &mut res.data))
            }
            SupportedCodeBlockSyntax::ConundrumAi => Ok(get_ai_parsing_request_phase_1_content(self, &mut res.data)),
            _ => {
                if res.data.ignore_all_parsers {
                    Ok(self.full_match.clone())
                } else if res.is_markdown() {
                    self.to_markdown(res)
                } else if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
                    self.to_plain_text(res)
                } else if res.contains_modifier(&ConundrumModifier::CodeBlocksAsIs) {
                    Ok(self.full_match.clone())
                } else {
                    self.to_mdx_component(res)
                }
            }
        }
    }
}

impl MdxComponentResult for ParsedCodeBlock {
    fn to_mdx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        match self.language {
            SupportedCodeBlockSyntax::Dictionary => {
                if res.should_ignore_parser(&ParserId::Dictionary) {
                    return Ok(self.full_match.clone());
                }

                // Extract the metadata or provide a fallback
                Ok(get_dictionary_content(self, &mut res.data))
            }
            SupportedCodeBlockSyntax::ConundrumAi => Ok(get_ai_parsing_request_phase_1_content(self, &mut res.data)),
            _ => self.to_html_js_component(res),
        }
    }
}

impl ConundrumParser<ParsedCodeBlock> for ParsedCodeBlock {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<ParsedCodeBlock> {
        let ((language, meta_opt, raw_content, tick_length), full_match) =
            (|input: &mut ConundrumInput<'a>| {
                let cp = input.input.checkpoint();
                let ticks = take_while(1.., |c: char| c == '`').parse_next(input).inspect_err(|_| {
                                                                                      input.input.reset(&cp);
                                                                                  })?;

                let language =
                    take_while(1.., |c: char| c != ' ' && c != '\t' && c != '\n' && c != '\r').verify_map(|s: &'a str| {
                        let _s = SupportedCodeBlockSyntax::format_string_for_key(s);
                        SupportedCodeBlockSyntax::from_str(_s.as_str()).ok()
                    })
                    .parse_next(input)?;

                let meta_opt = combinator::opt(|input: &mut ConundrumInput<'a>| {
                                   let _ = space1.parse_next(input)?;
                                   let _ = literal("--").parse_next(input)?;
                                   let _ = space0.parse_next(input)?;
                                   take_till(0.., AsChar::is_newline).parse_next(input)
                               }).parse_next(input)
                                 .inspect_err(|_| {
                                     input.input.reset(&cp);
                                 })?;

                let _ = space0.parse_next(input).inspect_err(|_| {
                                                     input.input.reset(&cp);
                                                 })?;
                let _ = line_ending(input).inspect_err(|_| {
                                              input.input.reset(&cp);
                                          })?;

                let raw_content = take_until(0.., ticks).parse_next(input).map_err(|e| {
                                                                               println!("Error: {:#?}", e);
                                                                               input.input.reset(&cp);
                                                                               e
                                                                           })?;
                let _ = literal(ticks).parse_next(input).map_err(|e| {
                                                             println!("Error: {:#?}", e);
                                                             input.input.reset(&cp);
                                                             e
                                                         })?;

                Ok((language, meta_opt, raw_content, ticks.len()))
            }).with_taken()
              .parse_next(input)?;

        let meta_data = meta_opt.map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        Ok(ParsedCodeBlock { language,
                             meta_data,
                             depth: tick_length as u8,
                             content: raw_content.to_string(),
                             full_match: full_match.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '`'
    }
}

#[cfg(test)]
mod tests {

    use insta::assert_snapshot;

    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_codeblock_with_title() {
        let test_content = r#"```js -- title="my_webview_content.swift"
// MY_COMMENT: My comment here
```"#;
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res =
            ParsedCodeBlock::parse_input_string(&mut test_data).expect("Parses code block without throwing an error.");

        assert_snapshot!(res.content);

        let mut state = test_data.state.borrow_mut();

        let mdx_content =
            res.to_mdx_component(&mut state).expect("Compiles code block to mdx without throwing an error.");

        assert_snapshot!(mdx_content);
        // assert_eq!(result, 4);
    }
}
