use fluster_core_utilities::core_types::fluster_error::FlusterResult;
use fluster_pre_parser::parse::by_regex::parse_mdx_by_regex::{
    ParseMdxOptions, parse_mdx_string_by_regex,
};

#[uniffi::export(async_runtime = "tokio")]
pub async fn pre_parse_mdx(data: ParseMdxOptions) -> FlusterResult<Vec<u8>> {
    parse_mdx_string_by_regex(data).await
}
