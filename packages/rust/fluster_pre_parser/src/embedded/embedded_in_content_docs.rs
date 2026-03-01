use fluster_core_utilities::core_types::in_content_documentation_id::{
    InContentDocumentationFormat, InContentDocumentationId,
};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "src/embedded/in_content_docs/"]
pub struct EmbeddedInContentDocs;

pub static MDX_DOCUMENTATION_ERROR_TEXT: &str = r#"
## Error

An error occurred while gathering this documentation. If this continues, please report an issue.
"#;

impl EmbeddedInContentDocs {
    pub fn read_string(fp: &str) -> Option<String> {
        if let Some(res) = EmbeddedInContentDocs::get(fp)
            && let Ok(body) = std::str::from_utf8(res.data.as_ref())
        {
            Some(body.to_string())
        } else {
            None
        }
    }
    pub fn get_incontent_docs_by_id(
        id: InContentDocumentationId,
        format: InContentDocumentationFormat,
    ) -> String {
        if let Some(res) = EmbeddedInContentDocs::get(&id.to_embedded_file_name(format))
            && let Ok(body) = std::str::from_utf8(res.data.as_ref())
        {
            return body.to_string();
        }
        MDX_DOCUMENTATION_ERROR_TEXT.to_string()
    }
}
