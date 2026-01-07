use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    parse::by_regex::parse_mdx_by_regex::ParseMdxOptions,
    parsing_result::mdx_parsing_result::MdxParsingResult,
};

#[derive(Serialize, Deserialize, PartialEq, strum_macros::Display)]
pub enum ParserId {
    #[serde(rename = "tags")]
    #[strum(to_string = "tags")]
    Tags,
    #[serde(rename = "citations")]
    #[strum(to_string = "citations")]
    Citations,
    #[serde(rename = "dictionary")]
    #[strum(to_string = "dictionary")]
    Dictionary,
}

#[async_trait]
pub trait MdxParser: Sync {
    fn parser_id(&self) -> ParserId;
    async fn parse_async(&self, req: &ParseMdxOptions, result: &mut MdxParsingResult);
}
