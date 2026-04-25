use conundrum::{
    lang::runtime::{
        run_conundrum::{ParseConundrumOptions, run_conundrum},
        state::{parse_state::ConundrumCompileTarget, ui_params::UIParams},
    },
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};

use crate::utils::path_utils::get_workspace_root;
use std::{fs, path::Path};

pub async fn parse_initial_notes() {
    let root = get_workspace_root();
    let initial_note_paths_path =
        Path::new(&root).join("docs").join("initial_note_docs").join("initial_note_paths.json");

    let res = fs::read_to_string(initial_note_paths_path).expect("Reads initial note paths without throwing an error.");

    let initial_note_paths: Vec<String> =
        serde_json::from_str(&res).expect("Deserializes initial_note_paths without error.");

    let mut results: Vec<MdxParsingResult> = Vec::new();

    for p in initial_note_paths.iter() {
        println!("Parsing the initial note docs at {}", p);
        let _p = format!("{}.mdx",
                         Path::new(&root).join("docs")
                                         .join(p)
                                         .to_str()
                                         .expect("Converts path to string without throwing an error."));
        let file_content = fs::read_to_string(_p).expect("Failed to read mdx file.");
        // TODO: This will break things now with the syntex themes. We'll need to render
        // both dark and light versions and then pick the proper one at build
        // time, or just compile the fucking thing at runtime.
        let mut res = run_conundrum(ParseConundrumOptions::new(None,
                                                               file_content.clone(),
                                                               Vec::new(),
                                                               Vec::new(),
                                                               UIParams::default(),
                                                               ConundrumCompileTarget::Html, true)
                                    )
                                         .expect("Returns a vald result when a valid input was provided.");

        // Need to re-assign file_content to _content so that the front-matter is still
        // present during the seeding of the initial note data.
        res.content = file_content;

        if let Some(fm) = res.front_matter.clone() {
            if fm.user_defined_id.is_none() {
                println!("No user defined id found in {}", p);
            }
        } else {
            println!("No front matter found in {}", p);
        }

        results.push(res);
        // if res.front_matter.id
    }

    let output_path = Path::new(&root).join("docs").join("initial_note_docs").join("initial_notes_parsed_data.json");

    let json_string = serde_json::to_string(&results).expect("Serializes parsed data hashmap to json.");

    fs::write(output_path, json_string).expect("Writes output to file without throwing an error.");
}
