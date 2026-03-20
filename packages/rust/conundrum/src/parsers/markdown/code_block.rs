use fluster_core_utilities::core_types::syntax::parser_ids::ParserId;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use winnow::{
    Parser,
    ascii::{line_ending, space0, space1},
    combinator,
    token::{literal, take_till, take_until, take_while},
};

use crate::{
    lang::runtime::traits::{conundrum_input::ConundrumInput, mdx_component_result::MdxComponentResult},
    output::{
        output_components::{
            ai_parsing_request_phase_1::get_ai_parsing_request_phase_1_content::get_ai_parsing_request_phase_1_content,
            dictionary_entry::get_dictionary_entry_content::get_dictionary_content,
        },
        parsing_result::mdx_parsing_result::MdxParsingResult,
    },
    parsers::parser_trait::ConundrumParser,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct ParsedCodeBlock {
    pub language: String,
    pub meta_data: Option<String>,
    pub content: String,
    pub full_match: String,
}

impl MdxComponentResult for ParsedCodeBlock {
    fn to_mdx_component(&self, res: &mut MdxParsingResult) -> String {
        if res.ignore_all_parsers {
            return self.full_match.clone();
        }
        match self.language.as_str() {
            "dictionary" => {
                if res.front_matter.as_ref().is_some_and(|fm| {
                                                fm.ignored_parsers
                                                  .iter()
                                                  .any(|x| x == &ParserId::Dictionary.to_string())
                                            })
                {
                    return self.full_match.clone();
                }

                // Extract the metadata or provide a fallback
                get_dictionary_content(self, res)
            }
            "fluster-ai" => get_ai_parsing_request_phase_1_content(self, res),
            _ => self.full_match.clone(),
        }
    }
}

impl ConundrumParser<ParsedCodeBlock> for ParsedCodeBlock {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> winnow::ModalResult<ParsedCodeBlock> {
        let ((language, meta_opt, raw_content), full_match) =
            (|input: &mut ConundrumInput<'a>| {
                let ticks = take_while(1.., |c: char| c == '`').parse_next(input)?;
                let language =
                    take_while(1.., |c: char| c != ' ' && c != '\t' && c != '\n' && c != '\r').parse_next(input)?;

                let meta_opt = combinator::opt(|input: &mut ConundrumInput<'a>| {
                                   let _ = space1.parse_next(input)?;
                                   let _ = literal("--").parse_next(input)?;
                                   let _ = space0.parse_next(input)?;
                                   take_till(0.., ('\n', '\r')).parse_next(input)
                               }).parse_next(input)?;

                let _ = space0.parse_next(input)?;
                let _ = line_ending(input)?;

                let raw_content = take_until(0.., ticks).parse_next(input)?;
                let _ = literal(ticks).parse_next(input)?;

                Ok((language, meta_opt, raw_content))
            }).with_taken()
              .parse_next(input)?;

        let meta_data = meta_opt.map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        Ok(ParsedCodeBlock { language: language.to_string(),
                             meta_data,
                             content: raw_content.to_string(),
                             full_match: full_match.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '`'
    }
}
