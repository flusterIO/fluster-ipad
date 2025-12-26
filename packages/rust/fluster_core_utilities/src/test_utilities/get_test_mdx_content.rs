use std::{env, fs, path};

pub fn get_welcome_to_fluster_content() -> String {
    let root = env::var("FLUSTER_IOS_ROOT").expect("Cannot continue wthout a FLUSTER_IOS_ROOT environment variable set to the root of your workspace.");
    let p = path::Path::new(&root)
        .join("docs")
        .join("initial_note_docs")
        .join("welcome_to_fluster.mdx");
    fs::read_to_string(p).expect("Could not read test mdx content.")
}

/// Get the note that describes the model's content
pub fn get_model_note_content() -> String {
    let root = env::var("FLUSTER_IOS_ROOT").expect("Cannot continue wthout a FLUSTER_IOS_ROOT environment variable set to the root of your workspace.");
    let p = path::Path::new(&root)
        .join("docs")
        .join("initial_note_docs")
        .join("on_the_gravitational_nature_of_time.mdx");
    fs::read_to_string(p).expect("Could not read test mdx content.")
}
