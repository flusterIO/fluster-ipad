use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::parsing_result::mdx_parsing_result::MdxParsingResult;

#[derive(Serialize, Deserialize, PartialEq, strum_macros::Display)]
pub enum ParserId {
    #[serde(rename = "tags")]
    #[strum(to_string = "tags")]
    Tags,
}

#[async_trait]
pub trait MdxParser: Sync {
    fn parser_id(&self) -> ParserId;
    async fn parse_async(&self, result: &mut MdxParsingResult);
}
