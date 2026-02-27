use fluster_core_utilities::core_types::fluster_error::{FlusterError, FlusterResult};
use tokio;

use fluster_pre_parser::{
    parse::by_regex::parse_mdx_by_regex::{ParseMdxOptions, parse_mdx_string_by_regex},
    parsing_result::mdx_parsing_result::MdxParsingResult,
};

#[uniffi::export(async_runtime = "tokio")]
pub async fn pre_parse_mdx(options: ParseMdxOptions) -> FlusterResult<MdxParsingResult> {
    let x = tokio::task::spawn(async { parse_mdx_string_by_regex(options).await }).await;
    x.map_err(|_| FlusterError::MdxParsingError)
}
