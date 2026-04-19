use crate::output::general::component_constants::{
    auto_inserted_component_name::AutoInsertedComponentName, component_ids::EmbeddableComponentId,
};
use dashmap::DashMap;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(strum_macros::Display, Serialize, Deserialize, Clone)]
pub enum WebGlueCodeGeneralFiles {
    #[serde(rename = "styles.css")]
    #[strum(to_string = "styles.css")]
    Styles,
    #[serde(rename = "katex.min.css")]
    #[strum(to_string = "katex.min.css")]
    KatexCss,
}

#[derive(Clone)]
pub enum AnyGlueCodeKey {
    AutoInserted(AutoInsertedComponentName),
    Embeddable(EmbeddableComponentId),
    General(WebGlueCodeGeneralFiles),
}

lazy_static! {
    pub static ref JS_GLUE_CODE_MAP: DashMap<String, &'static str> = {
        let map = DashMap::new();
        map.insert(WebGlueCodeGeneralFiles::Styles.to_string(), include_str!("../standalone/conundrum.css"));
        map.insert(WebGlueCodeGeneralFiles::KatexCss.to_string(),
                   include_str!("../glue/component_glue/other/katex.min.css"));
        map.insert(AutoInsertedComponentName::AutoInsertedCodeBlock.to_string(),
                   include_str!("./component_glue_output/code_block.js"));
        map.insert(AutoInsertedComponentName::NoteLink.to_string(),
                   include_str!("./component_glue_output/note_link.js"));
        map
    };
}

impl JS_GLUE_CODE_MAP {
    pub fn get_by_key(k: AnyGlueCodeKey) -> Option<String> {
        let key = match k {
            AnyGlueCodeKey::AutoInserted(n) => n.to_string(),
            AnyGlueCodeKey::Embeddable(n) => n.to_string(),
            AnyGlueCodeKey::General(n) => n.to_string(),
        };

        if let Some(res) = JS_GLUE_CODE_MAP.get(&key) {
            let s = res.value().to_string();
            Some(s)
        } else {
            None
        }
    }
}
