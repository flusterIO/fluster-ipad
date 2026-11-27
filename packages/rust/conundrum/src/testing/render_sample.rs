use askama::Template;
use std::{fs, path::Path};

pub fn write_test_sandbox_output_for_dev(content: &str) {
    let root = std::env::var_os("FLUSTER_IOS_ROOT")
        .expect("FLUSTER_IOS_ROOT must point to the monorepo root when writing test output");
    let p = Path::new(&root);
    let x = p.join("packages").join("rust").join("conundrum").join("test_output.html");

    fs::write(x, content).expect("Failed to write test output");
}
