use std::{rc::Rc, str::FromStr, sync::Arc};

use askama::Template;
use serde::{Deserialize, Serialize};
use syntect_assets::assets::HighlightingAssets;
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
                parse_state::{ConundrumCompileTarget, ConundrumModifier, ParseState},
            },
            traits::{
                conundrum_input::{ArcState, ConundrumInput, get_conundrum_input},
                fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult,
                markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
                state_modifier::ConundrumStateModifier,
            },
        },
    },
    output::{
        general::component_constants::{
            any_component_id::AnyComponentName, auto_inserted_component_name::AutoInsertedComponentName,
            parser_ids::ParserId,
        },
        html::dom::dom_id::DOMId,
        output_components::{
            ai_parsing_request_phase_1::get_ai_parsing_request_phase_1_content::get_ai_parsing_request_phase_1_content,
            dictionary_entry::get_dictionary_entry_content::get_dictionary_content,
        },
        parsing_result::{
            ai_serialization_request::AiSerializationRequestPhase1, dictionary_result::DictionaryEntryResult,
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
    pub id: DOMId,
}

impl ConundrumStateModifier<ParsedCodeBlock> for ParsedCodeBlock {
    fn set_state(res: ArcState, data: Option<ParsedCodeBlock>) {
        let cb = data.unwrap();
        if cb.language == SupportedCodeBlockSyntax::Dictionary {
            let mut state = res.write_arc();
            let term = cb.meta_data.as_deref().unwrap_or("Untitled");
            state.data.dictionary_entries.push(DictionaryEntryResult { label: term.to_string(),
                                                                       body: cb.content.clone() });
            drop(state);
        } else if cb.language == SupportedCodeBlockSyntax::ConundrumAi {
            let mut state = res.write_arc();
            state.data.ai_secondary_parse_requests.push(AiSerializationRequestPhase1 { parsing_result: cb.clone() });
            drop(state);
        }
    }
}

impl ParsedCodeBlock {
    pub fn get_meta_data(&self) -> Option<ConundrumObject> {
        if let Some(meta) = &self.meta_data {
            let input = &mut get_conundrum_input(meta.as_str(), ParseState::default());
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

    pub fn get_highlighted_content(&self,
                                   theme: SupportedCodeBlockTheme,
                                   hl: Arc<HighlightingAssets>)
                                   -> ConundrumModalResult<String> {
        render_general_codeblock_to_html(RenderCodeToHtmlReq { code: GeneralPresentationCodeBlock { content:
                                                                                                        self.content
                                                                                                            .clone(),
                                                                                                    lang:
                                                                                                        self.language
                                                                                                            .clone(),
                                                                                                    theme:
                                                                                                        Some(theme),
                                                                                                    inline: false,
                                                                                                    assets: hl } })
    }
}

impl PlainTextComponentResult for ParsedCodeBlock {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.to_markdown(res)
    }
}

impl MarkdownComponentResult for ParsedCodeBlock {
    fn to_markdown(&self, _: ArcState) -> ConundrumModalResult<String> {
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

/// Move this id to the parsing stage!
impl HtmlJsComponentResult for ParsedCodeBlock {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let read_state = res.read_arc();
        let assets = Arc::clone(&read_state.highlight_assets);
        let code_string =
            self.get_highlighted_content(read_state.ui_params.syntax_theme.clone().unwrap_or_default(), assets)?;
        let template = CodeBlockHTMLTemplate::new(code_string,
                                                  self.get_title(),
                                                  self.id.clone(),
                                                  &self.language,
                                                  &read_state.ui_params.dark_mode);
        template.render().map_err(|e| {
                             eprintln!("Error: {:#?}", e);
                             ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("")))
                         })
    }
}

impl ConundrumComponentResult for ParsedCodeBlock {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        match self.language {
            SupportedCodeBlockSyntax::Dictionary => {
                if state.should_ignore_parser(&ParserId::Dictionary) {
                    return Ok(self.full_match.clone());
                }

                drop(state);
                // Extract the metadata or provide a fallback
                Ok(get_dictionary_content(self, Arc::clone(&res)))
            }
            SupportedCodeBlockSyntax::ConundrumAi => Ok(get_ai_parsing_request_phase_1_content(self)),
            _ => {
                if state.data.ignore_all_parsers {
                    drop(state);
                    Ok(self.full_match.clone())
                } else if state.targets_markdown() {
                    drop(state);
                    self.to_markdown(res)
                } else if state.compile_target == ConundrumCompileTarget::PlainText {
                    drop(state);
                    self.to_plain_text(res)
                } else if state.contains_modifier(&ConundrumModifier::CodeBlocksAsIs) {
                    drop(state);
                    Ok(self.full_match.clone())
                } else {
                    drop(state);
                    self.to_mdx_component(res)
                }
            }
        }
    }
}

impl MdxComponentResult for ParsedCodeBlock {
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        match self.language {
            SupportedCodeBlockSyntax::Dictionary => {
                if state.should_ignore_parser(&ParserId::Dictionary) {
                    return Ok(self.full_match.clone());
                } else {
                    drop(state);
                }
                // Extract the metadata or provide a fallback
                Ok(get_dictionary_content(self, Arc::clone(&res)))
            }
            SupportedCodeBlockSyntax::ConundrumAi => Ok(get_ai_parsing_request_phase_1_content(self)),
            _ => {
                drop(state);
                self.to_html_js_component(res)
            }
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
        let mut state = input.state.write_arc();
        let id = state.dom.new_id();
        state.data.append_embeddable_component(&AnyComponentName::AutoInserted(AutoInsertedComponentName::AutoInsertedCodeBlock));
        drop(state);
        Ok(ParsedCodeBlock { language,
                             meta_data,
                             id,
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

        let mdx_content = res.to_mdx_component(Arc::clone(&test_data.state))
                             .expect("Compiles code block to mdx without throwing an error.");

        assert_snapshot!(mdx_content);
        // assert_eq!(result, 4);
    }
}
