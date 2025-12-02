use fluster_core_utilities::core_types::fluster_error::FlusterResult;
use fluster_pre_parser::{
    parse::by_regex::parse_mdx_by_regex::{ParseMdxOptions, parse_mdx_string_by_regex},
    parsing_result::mdx_parsing_result::MdxParsingResult,
};

#[uniffi::export(async_runtime = "tokio")]
pub async fn pre_parse_mdx(data: ParseMdxOptions) -> FlusterResult<MdxParsingResult> {
    // let mut res = MdxParsingResult::from_initial_mdx_content(&data.content);
    parse_mdx_string_by_regex(data).await
}
