use std::{env, fs};

pub fn get_test_content(fp: &str) -> String {
    let manifest_dir_str = env!("CARGO_MANIFEST_DIR");
    let file_path = std::path::Path::new(manifest_dir_str).join("src").join("testing").join("test_input").join(fp);
    assert!(fs::exists(&file_path).is_ok_and(|x| x), "Test content exists...");

    fs::read_to_string(&file_path).expect("Failed to reada test content")
}
