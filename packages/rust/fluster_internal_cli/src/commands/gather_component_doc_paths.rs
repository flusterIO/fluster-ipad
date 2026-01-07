use ignore::{WalkBuilder, types::TypesBuilder};
use serde::Serialize;

use crate::utils::path_utils::get_workspace_root;

#[derive(Serialize)]
struct ComponentDocsData {
    paths: Vec<String>,
}

const IGNORE_DIRS: [&str; 2] = ["node_modules/", "target/"];

pub fn run() {
    let root = get_workspace_root();
    let mut b = WalkBuilder::new(&root);
    let mut t = TypesBuilder::new();
    t.add_def("fluster:**/*.fluster_component.json")
        .expect("Failed to add fluster path type");
    let walker = b.types(t.select("fluster").build().unwrap());

    for d in IGNORE_DIRS {
        walker.add_ignore(d);
    }

    let mut paths: Vec<String> = Vec::new();

    for result in walker.build() {
        match result {
            Ok(entry) => {
                // This code will not be executed for files within node_modules
                println!("{}", entry.path().display());
                if entry.path().is_file() {
                    paths.push(entry.path().display().to_string());
                }
            }
            Err(err) => eprintln!("ERROR: {}", err),
        }
    }

    let res = ComponentDocsData { paths };
    let s = serde_json::to_string(&res).expect("Failed to serialize path data.");
    std::fs::write(
        std::path::Path::new(&root)
            .join("docs")
            .join("component_docs")
            .join(".component_doc_paths.json"),
        s,
    )
    .expect("Failed to write path output to file.");
}
