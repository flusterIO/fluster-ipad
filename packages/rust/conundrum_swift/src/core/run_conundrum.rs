use conundrum::{
    lang::runtime::run_conundrum::{run_conundrum, ParseMdxOptions},
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};
use fluster_core_utilities::core_types::fluster_error::{FlusterError, FlusterResult};
use tokio::task::spawn;

#[uniffi::export(async_runtime = "tokio")]
pub async fn run_conundrum_swift(options: ParseMdxOptions) -> FlusterResult<MdxParsingResult> {
    let x = spawn(async move { run_conundrum(options).await }).await;
    x.map_err(|_| FlusterError::ConundrumParsingError)
}
