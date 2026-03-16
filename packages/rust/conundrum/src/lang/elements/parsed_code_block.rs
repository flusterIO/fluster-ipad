use fluster_core_utilities::core_types::syntax::parser_ids::ParserId;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    lang::runtime::traits::mdx_component_result::MdxComponentResult,
    output::{
        output_components::{
            ai_parsing_request_phase_1::get_ai_parsing_request_phase_1_content::get_ai_parsing_request_phase_1_content,
            dictionary_entry::get_dictionary_entry_content::get_dictionary_content,
        },
        parsing_result::mdx_parsing_result::MdxParsingResult,
    },
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
                if let Some(fm) = &res.front_matter {
                    if fm
                        .ignored_parsers
                        .iter()
                        .any(|x| x == &ParserId::Dictionary.to_string())
                    {
                        return self.full_match.clone();
                    }
                }
                // Extract the metadata or provide a fallback
                get_dictionary_content(&self, res)
            }
            "fluster-ai" => get_ai_parsing_request_phase_1_content(&self, res),
            _ => {
                if let Some(fm) = &res.front_matter {
                    if fm
                        .ignored_parsers
                        .iter()
                        .any(|x| x == &ParserId::AiTrigger.to_string())
                    {
                        return self.full_match.clone();
                    }
                }
                // For standard code blocks (like tsx, rust, etc.), leave them exactly as they are and
                // let mdx handle it for now.
                self.full_match.clone()
            }
        }
    }
}
