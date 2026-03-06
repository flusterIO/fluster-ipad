use async_trait::async_trait;
use fluster_core_utilities::core_types::syntax::parser_ids::ParserId;

use crate::{
    parse::by_regex::parse_mdx_by_regex::ParseMdxOptions,
    parsing_result::mdx_parsing_result::MdxParsingResult,
};

#[async_trait]
pub trait MdxParser: Sync {
    fn parser_id(&self) -> ParserId;
    async fn parse_async(&self, req: &ParseMdxOptions, result: &mut MdxParsingResult);
}
