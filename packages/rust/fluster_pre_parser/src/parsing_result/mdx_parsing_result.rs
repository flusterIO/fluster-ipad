use fluster_core_utilities::core_types::fluster_error::FlusterError;
use gray_matter::{Matter, engine::YAML};
use serde::{Deserialize, Serialize};

use crate::parsing_result::{front_matter::FrontMatterResult, tag_result::TagResult};

#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct MdxParsingResult {
    pub content: String,
    pub tags: Vec<TagResult>,
    pub front_matter: Option<FrontMatterResult>,
}

impl MdxParsingResult {
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
