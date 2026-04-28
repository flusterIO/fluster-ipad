use conundrum::output::{
    general::component_constants::component_ids::EmbeddableComponentId,
    html::glue::component_glue_manager::{AnyComponentKey, WEB_GLUE_CODE_MAP},
};
use strum::IntoEnumIterator;

use crate::utils::get_workspace_root_duplicate::get_workspace_root;

pub fn get_combined_css_glue() -> String {
    let mut s = String::from("");
    for k in EmbeddableComponentId::iter() {
        if let Some(js) = WEB_GLUE_CODE_MAP::get_css_string_by_key(AnyComponentKey::Embeddable(k)) {
            s += format!("{}\n", js).as_str();
        }
    }
    s
}

pub fn get_combined_javascript_glue() -> String {
    let mut s = String::from("");
    for k in EmbeddableComponentId::iter() {
        if let Some(js) = WEB_GLUE_CODE_MAP::get_js_string_by_key(AnyComponentKey::Embeddable(k)) {
            s += format!("{}\n", js).as_str();
        }
    }
    s
}

pub fn write_combined_javascript_glue_code() {
    let content = get_combined_javascript_glue();
    let root = get_workspace_root();
    let output_path = std::path::Path::new(&root).join("packages")
                                                 .join("rust")
                                                 .join("conundrum_web_assets")
                                                 .join("src")
                                                 .join("js")
                                                 .join("combined_component_glue.js");
    std::fs::write(output_path, content).expect("Writes component glue javascript without throwing an error.");
}

pub fn write_combined_css_glue_code() {
    let content = get_combined_css_glue();
    let root = get_workspace_root();
    let output_path = std::path::Path::new(&root).join("packages")
                                                 .join("rust")
                                                 .join("conundrum_web_assets")
                                                 .join("src")
                                                 .join("css")
                                                 .join("combined_component_glue.css");
    std::fs::write(output_path, content).expect("Writes component glue javascript without throwing an error.");
}
