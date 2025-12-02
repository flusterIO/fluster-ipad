use async_trait::async_trait;

use crate::parsing_result::mdx_parsing_result::MdxParsingResult;

#[derive(PartialEq)]
#[repr(u8)]
#[derive(uniffi::Enum)]
pub enum ParserId {
    Tag = 0,
}

#[async_trait]
pub trait MdxParser: Sync {
    fn parser_id(&self) -> ParserId;
    async fn parse_async(&self, result: &mut MdxParsingResult);
}
