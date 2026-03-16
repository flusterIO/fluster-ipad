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
        match self.language.as_str() {
            "dictionary" => {
                // Extract the metadata or provide a fallback
                get_dictionary_content(&self, res)
            }
            "fluster-ai" => get_ai_parsing_request_phase_1_content(&self, res),
            _ => {
                // For standard code blocks (like tsx, rust, etc.), leave them exactly as they are and
                // let mdx handle it for now.
                self.full_match.clone()
            }
        }
    }
}
