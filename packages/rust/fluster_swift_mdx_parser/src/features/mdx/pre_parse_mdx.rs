use conundrum::{
    embedded::embedded_in_content_docs::EmbeddedInContentDocs,
    lang::runtime::run_conundrum::{ParseMdxOptions, run_conundrum},
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};
use fluster_core_utilities::core_types::{
    documentation_constants::in_content_documentation_id::{
        InContentDocumentationFormat, InContentDocumentationId,
    },
    fluster_error::{FlusterError, FlusterResult},
};
use tokio::task::spawn;

#[uniffi::export(async_runtime = "tokio")]
pub async fn pre_parse_mdx(options: ParseMdxOptions) -> FlusterResult<MdxParsingResult> {
    let x = spawn(async move { run_conundrum(options).await }).await;
    x.map_err(|_| FlusterError::MdxParsingError)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn get_embedded_docs_by_id(
    id: InContentDocumentationId,
    format: InContentDocumentationFormat,
) -> String {
    EmbeddedInContentDocs::get_incontent_docs_by_id(&id, &format)
}
