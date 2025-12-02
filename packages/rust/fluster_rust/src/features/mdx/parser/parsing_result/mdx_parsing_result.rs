use crate::features::mdx::parser::parsing_result::tag_result::TagResult;

#[derive(uniffi::Object)]
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
