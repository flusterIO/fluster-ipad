use rust_embed::RustEmbed;

use crate::embedded::in_content_documentation_id::{InContentDocumentationFormat, InContentDocumentationId};


/// A simple lookup table for the embedded documentation that isn't related to any specific
/// component.
#[derive(RustEmbed)]
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

    pub fn get_incontent_docs_by_id(id: &InContentDocumentationId, format: &InContentDocumentationFormat) -> String {
        if let Some(res) = EmbeddedInContentDocs::get(&id.to_embedded_file_name(format))
           && let Ok(body) = std::str::from_utf8(res.data.as_ref())
        {
            return body.to_string();
        }
        MDX_DOCUMENTATION_ERROR_TEXT.to_string()
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn gets_in_content_docs_by_id() {
        for format_name in InContentDocumentationFormat::iter() {
            for id in InContentDocumentationId::iter() {
                let res = EmbeddedInContentDocs::get_incontent_docs_by_id(&id, &format_name);
                assert!(!res.is_empty(), "Found non-empty content for the component with the id {}", id);
                assert!(res != MDX_DOCUMENTATION_ERROR_TEXT,
                        "Received proper text content for the component with the id '{}' and format {}",
                        id,
                        format_name);
            }
        }
        // assert_eq!(result, 4);
    }
}
