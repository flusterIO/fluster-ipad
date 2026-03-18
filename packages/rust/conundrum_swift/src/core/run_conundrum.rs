use conundrum::{
    lang::runtime::run_conundrum::{run_conundrum as run_conundrum_swift, ParseMdxOptions},
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};
use fluster_core_utilities::core_types::fluster_error::{FlusterError, FlusterResult};
use tokio::task::spawn;

#[uniffi::export(async_runtime = "tokio")]
pub async fn run_conundrum(options: ParseMdxOptions) -> FlusterResult<MdxParsingResult> {
    let x = spawn(async move { run_conundrum_swift(options).await }).await;
    x.map_err(|x| {
         println!("Error: {:#?}", x);
         FlusterError::ConundrumParsingError
     })
}
