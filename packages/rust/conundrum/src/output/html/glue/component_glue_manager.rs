use rust_embed::Embed;

use crate::output::general::component_constants::{
    any_component_id::AnyComponentName, auto_inserted_component_name::AutoInsertedComponentName,
};

#[derive(Embed)]
#[folder = "src/output/html/glue/component_glue_output"]
pub struct EmbeddedGlueCode;

impl EmbeddedGlueCode {
    fn get_by_fp(fp: &str) -> Option<String> {
        if let Some(res) = EmbeddedGlueCode::get(fp)
           && let Ok(body) = std::str::from_utf8(res.data.as_ref())
        {
            Some(body.to_string())
        } else {
            None
        }
    }

    pub fn get_embedded_js(name: AnyComponentName) -> Option<String> {
        match name {
            AnyComponentName::AutoInserted(n) => match n {
                AutoInsertedComponentName::AutoInsertedCodeBlock => EmbeddedGlueCode::get_by_fp("code_block.js"),
                _ => None,
            },
            _ => None,
        }
    }
}
