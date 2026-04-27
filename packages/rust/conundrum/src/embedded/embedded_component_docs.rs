use rust_embed::Embed;

use crate::{
    embedded::{
        embedded_in_content_docs::MDX_DOCUMENTATION_ERROR_TEXT,
        in_content_documentation_id::InContentDocumentationFormat,
    },
    output::general::component_constants::component_ids::EmbeddableComponentId,
};

/// A simple lookup table for the embedded component-specific documentation.
#[derive(Embed)]
#[folder = "src/embedded/component_docs/"]
pub struct EmbeddedComponentDocs;

impl EmbeddedComponentDocs {
    pub fn read_string(fp: &str) -> Option<String> {
        if let Some(res) = EmbeddedComponentDocs::get(fp)
           && let Ok(body) = std::str::from_utf8(res.data.as_ref())
        {
            Some(body.to_string())
        } else {
            None
        }
    }

    pub fn get_incontent_docs_by_id(id: &EmbeddableComponentId, format: &InContentDocumentationFormat) -> String {
        if let Some(res) = EmbeddedComponentDocs::get(&id.to_embedded_file_name(format))
           && let Ok(body) = std::str::from_utf8(res.data.as_ref())
        {
            return body.to_string();
        }
        MDX_DOCUMENTATION_ERROR_TEXT.to_string()
    }
}
