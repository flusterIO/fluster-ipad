use conundrum::{
    lang::runtime::{
        run_conundrum::{run_conundrum as run_conundrum_swift, ParseConundrumOptions},
        state::conundrum_error_variant::{ConundrumErrorVariant, ConundrumResult},
    },
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};
use tokio::task::spawn;

#[uniffi::export(async_runtime = "tokio")]
pub async fn run_conundrum(options: ParseConundrumOptions) -> ConundrumResult<MdxParsingResult> {
    spawn(async move { run_conundrum_swift(options) }).await.map_err(|x| {
                                                                 println!("Threading Error: {:#?}", x);
                                                                 ConundrumErrorVariant::MultiThreadingError
                                                             })?
}
