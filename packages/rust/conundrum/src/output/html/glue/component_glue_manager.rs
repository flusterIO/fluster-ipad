use dashmap::DashMap;
use lazy_static::lazy_static;
use rust_embed::Embed;
use serde::{Deserialize, Serialize};

use crate::output::general::component_constants::{
    any_component_id::AnyComponentName, auto_inserted_component_name::AutoInsertedComponentName,
    component_ids::EmbeddableComponentId,
};

pub enum AnyGlueCodeKey {
    AutoInserted(AutoInsertedComponentName),
    Embeddable(EmbeddableComponentId),
}

#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum WebGlueCodeGeneralFiles {
    #[serde(rename = "styles.css")]
    #[strum(to_string = "styles.css")]
    Styles,
}

#[derive(Embed)]
#[folder = "src/output/html/glue/component_glue_output"]
pub struct EmbeddedGlueCode;

lazy_static! {
    pub static ref JS_GLUE_CODE_MAP: DashMap<String, &'static str> = {
        let map = DashMap::new();
        map.insert(WebGlueCodeGeneralFiles::Styles.to_string(), include_str!("../standalone/conundrum.css"));
        map.insert(AutoInsertedComponentName::AutoInsertedCodeBlock.to_string(),
                   include_str!("./component_glue_output/code_block.js"));
        map
    };
}

impl JS_GLUE_CODE_MAP {
    pub fn get_by_key(&self, k: AnyGlueCodeKey) -> Option<String> {
        let key = match k {
            AnyGlueCodeKey::AutoInserted(n) => n.to_string(),
            AnyGlueCodeKey::Embeddable(n) => n.to_string(),
        };

        let res = self.get(&key);
        res.map(|n| n.to_string())
    }
}

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
