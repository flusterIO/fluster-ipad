use crate::features::mdx::parser::{
    by_regex::regex_parsers::mdx_parser::{MdxParser, ParserId},
    parsing_result::mdx_parsing_result::MdxParsingResult,
};
use async_trait::async_trait;

pub struct TagRegexParser;

#[async_trait]
impl MdxParser for TagRegexParser {
    fn parser_id(&self) -> ParserId {
        ParserId::Tag
    }
    async fn parse_async(&self, results: &mut MdxParsingResult) {
        // let regex = RegexParser
        // For now, just return the markdown as is.
        // Actual parsing logic will be added later.
        // markdown.to_string()
    }
}
