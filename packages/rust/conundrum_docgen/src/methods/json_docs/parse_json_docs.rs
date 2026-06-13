use rustdoc_types::Item;

use crate::{
    errors::DocGenResult, methods::json_docs::get_documentation_string::get_documentation_string,
    workspace_utils::get_workspace_root_duplicate::get_workspace_root,
};

pub const USER_DOCUMENTATION_MARKER: &str = "@userfacing";

/// Returns a modified string if the documentation should be parsed further,
/// otherwise returns None.
fn process_documentation(value: String) -> Option<String> {
    let contains_flag = value.contains(USER_DOCUMENTATION_MARKER);
    if contains_flag {
        Some(value.replace(USER_DOCUMENTATION_MARKER, ""))
    } else {
        None
    }
}

pub fn parse_json_docs(json_path: &str) -> DocGenResult<()> {
    let p = std::path::Path::new(json_path);
    let d = std::fs::read_to_string(p).expect("Failed to read generated json documentation.");
    let crate_docs: rustdoc_types::Crate =
        serde_json::from_str(&d).expect("Failed to deserialize generatd json documentation.");
    let root = get_workspace_root();
    let generated_dir =
        std::path::Path::new(&root).join("packages").join("rust").join("conundrum_docgen").join("generated");

    let mut docs_entry: Option<Item> = None;
    for (entry_id, item) in crate_docs.clone().index.iter() {
        if item.name.as_ref().cloned().is_some_and(|s| s.contains("DocGenTypes")) {
            println!("It's Included!!!");
        }
        if item.name.as_ref().cloned().is_some_and(|s| &s == "DocGenTypes") {
            docs_entry = Some(item.clone());
        } else if let Some(item_name) = item.name.clone() {
            println!("Skipped: {:#?}", item_name);
        }
    }
    if docs_entry.is_none() {
        panic!("Could dnot locate main documentnation entry point enum");
    }
    Ok(())
}
