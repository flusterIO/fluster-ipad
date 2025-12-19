use crate::utils::path_utils::get_workspace_root;
use fluster_pre_parser::{
    parse::by_regex::parse_mdx_by_regex::{ParseMdxOptions, parse_mdx_string_to_mdx_result},
    parsing_result::mdx_parsing_result::MdxParsingResult,
};
use std::{fs, path::Path};

pub async fn parse_initial_notes() {
    let root = get_workspace_root();
    let initial_note_paths_path = Path::new(&root)
        .join("docs")
        .join("initial_note_docs")
        .join("initial_note_paths.json");

    let res = fs::read_to_string(initial_note_paths_path)
        .expect("Reads initial note paths without throwing an error.");

    let initial_note_paths: Vec<String> =
        serde_json::from_str(&res).expect("Deserializes initial_note_paths without error.");

    let mut results: Vec<MdxParsingResult> = Vec::new();

    for p in initial_note_paths.iter() {
        let _p = format!(
            "{}.mdx",
            Path::new(&root)
                .join("docs")
                .join(p)
                .to_str()
                .expect("Converts path to string without throwing an error.")
        );
        let file_content = fs::read_to_string(_p).expect("Failed to read mdx file.");
        let mut res = parse_mdx_string_to_mdx_result(&ParseMdxOptions {
            content: file_content.clone(),
            citations: Vec::new(),
        })
        .await;

        // Need to re-assign file_content to _content so that the front-matter is still present
        // during the seeding of the initial note data.
        res.content = file_content;

        if let Some(fm) = res.front_matter.clone() {
            if fm.user_defined_id.is_none() {
                println!("No front matter found in {}", p);
            }
        } else {
            println!("No user defined id found in {}", p);
        }

        results.push(res);
        // if res.front_matter.id
    }

    let output_path = Path::new(&root)
        .join("docs")
        .join("initial_note_docs")
        .join("initial_notes_parsed_data.json");

    let json_string =
        serde_json::to_string(&results).expect("Serializes parsed data hashmap to json.");

    fs::write(output_path, json_string).expect("Writes output to file without throwing an error.");
}
