use std::ops::Deref;

use async_trait::async_trait;

use crate::features::mdx::parser::parsing_result::mdx_parsing_result::MdxParsingResult;

#[derive(PartialEq)]
pub enum ParserId {
    Tag,
}

#[async_trait]
pub trait MdxParser: Sync {
    fn parser_id(&self) -> ParserId;
    async fn parse_async(&self, results: &mut MdxParsingResult);
}
