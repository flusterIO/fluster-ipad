use conundrum::ecosystem::{error_handling::db_error::{DatabaseError, DatabaseResult}};
use conundrum::lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum};
use crate::vector::database::db_traits::async_traits::actionable_request::ActionableRequest;
use conundrum::output::parsing_result::mdx_parsing_result::MdxParsingResult;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct CompileCdrmRequest {
    pub opts: ParseConundrumOptions
}



impl ActionableRequest<MdxParsingResult> for CompileCdrmRequest {
    async fn execute_request(&self) -> DatabaseResult<MdxParsingResult> {
         run_conundrum(self.opts.clone())
            .map_err(|e| {
                log::error!("Error: {:?}", e);
                DatabaseError::ConundrumError(e)
            })
    }
}
