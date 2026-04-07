use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use winnow::{
    Parser,
    ascii::{line_ending, space0, space1},
    combinator,
    stream::Stream,
    token::{literal, take_till, take_until, take_while},
};

use crate::{
    lang::runtime::{
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
    output::{
        general::component_constants::{auto_inserted_component_name::AutoInsertedComponentName, parser_ids::ParserId},
        output_components::{
            ai_parsing_request_phase_1::get_ai_parsing_request_phase_1_content::get_ai_parsing_request_phase_1_content,
            dictionary_entry::get_dictionary_entry_content::get_dictionary_content,
        },
    },
    parsers::{conundrum::logic::string::conundrum_string::ConundrumString, parser_trait::ConundrumParser},
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct ParsedCodeBlock {
    pub language: ConundrumString,
    pub meta_data: Option<String>,
    pub depth: u8,
    pub content: String,
    pub full_match: String,
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
                   tick_string, self.language.0, self.content, tick_string
        ))
    }
}

impl ConundrumComponentResult for ParsedCodeBlock {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        match self.language.0.as_str() {
            "dictionary" => {
                if res.should_ignore_parser(&ParserId::Dictionary) {
                    return Ok(self.full_match.clone());
                }

                // Extract the metadata or provide a fallback
                Ok(get_dictionary_content(self, &mut res.data))
            }
            "fluster-ai" => Ok(get_ai_parsing_request_phase_1_content(self, &mut res.data)),
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
        match self.language.0.as_str() {
            "dictionary" => {
                if res.should_ignore_parser(&ParserId::Dictionary) {
                    return Ok(self.full_match.clone());
                }

                // Extract the metadata or provide a fallback
                Ok(get_dictionary_content(self, &mut res.data))
            }
            "fluster-ai" => Ok(get_ai_parsing_request_phase_1_content(self, &mut res.data)),
            _ => Ok(format!(
                            r#"<{} {}>
{}
</{}>"#,
                            AutoInsertedComponentName::AutoInsertedCodeBlock,
                            self.language
                                .to_jsx_prop_as_string("language")
                                .unwrap_or_else(|_| String::from(self.language.0.clone())),
                            self.content,
                            AutoInsertedComponentName::AutoInsertedCodeBlock,
            )),
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
                    take_while(1.., |c: char| c != ' ' && c != '\t' && c != '\n' && c != '\r').parse_next(input)?;

                let meta_opt = combinator::opt(|input: &mut ConundrumInput<'a>| {
                                   let _ = space1.parse_next(input)?;
                                   let _ = literal("--").parse_next(input)?;
                                   let _ = space0.parse_next(input)?;
                                   take_till(0.., ('\n', '\r')).parse_next(input)
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
        Ok(ParsedCodeBlock { language: ConundrumString(language.to_string()),
                             meta_data,
                             depth: tick_length as u8,
                             content: raw_content.to_string(),
                             full_match: full_match.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '`'
    }
}
