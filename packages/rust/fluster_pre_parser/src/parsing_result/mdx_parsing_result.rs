use serde::{Deserialize, Serialize};

use crate::parsing_result::tag_result::TagResult;

#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct MdxParsingResult {
    pub content: String,
    pub tags: Vec<TagResult>,
}

impl MdxParsingResult {
    pub fn from_initial_mdx_content(content: &str) -> MdxParsingResult {
        MdxParsingResult {
            content: content.to_string(),
            tags: Vec::new(),
        }
    }
}
