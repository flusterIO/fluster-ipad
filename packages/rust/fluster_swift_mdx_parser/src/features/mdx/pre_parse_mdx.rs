use fluster_core_utilities::core_types::{
    documentation_constants::in_content_documentation_id::{
        InContentDocumentationFormat, InContentDocumentationId,
    },
    fluster_error::{FlusterError, FlusterResult},
};
use tokio;

use fluster_pre_parser::{
    embedded::embedded_in_content_docs::EmbeddedInContentDocs,
    parse::by_regex::parse_mdx_by_regex::{ParseMdxOptions, parse_mdx_string_to_mdx_result},
    parsing_result::mdx_parsing_result::MdxParsingResult,
};

#[uniffi::export(async_runtime = "tokio")]
pub async fn pre_parse_mdx(options: ParseMdxOptions) -> FlusterResult<MdxParsingResult> {
    let x = tokio::task::spawn(async move { parse_mdx_string_to_mdx_result(&options).await }).await;
    x.map_err(|_| FlusterError::MdxParsingError)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn get_embedded_docs_by_id(
    id: InContentDocumentationId,
    format: InContentDocumentationFormat,
) -> String {
    EmbeddedInContentDocs::get_incontent_docs_by_id(&id, &format)
}
