use fluster_core_utilities::core_types::fluster_error::FlusterError;
use gray_matter::{Matter, engine::YAML};
use serde::{Deserialize, Serialize};

use crate::parsing_result::{
    citation_result::CitationResult, front_matter::FrontMatterResult, tag_result::TagResult,
};

#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct MdxParsingResult {
    pub content: String,
    pub tags: Vec<TagResult>,
    pub front_matter: Option<FrontMatterResult>,
    /// bibliography_string is a string representing the concatenated bibtex entries of all
    /// valid bibentries within the note, without duplicates and in the proper order.
    pub citations: Vec<CitationResult>,
}

impl MdxParsingResult {
    /// This will not return a parsed item, but rather an empty template that can be passed to a
    /// series of MdxParsers to apply the necessary properties and changes.
    pub fn from_initial_mdx_content(content: &str) -> MdxParsingResult {
        let matter = Matter::<YAML>::new();
        let data = matter
            .parse(content)
            .map_err(|e| {
                println!("Front Matter Error: {}", e);
                FlusterError::FrontMatterError
            })
            .ok();
        MdxParsingResult {
            content: match &data {
                Some(d) => d.content.clone(),
                None => "".to_string(),
            },
            citations: Vec::new(),
            tags: Vec::new(),
            front_matter: match data {
                Some(front_matter_data) => front_matter_data
                    .data
                    .map(FrontMatterResult::from_gray_matter),
                None => None,
            },
        }
    }
}
