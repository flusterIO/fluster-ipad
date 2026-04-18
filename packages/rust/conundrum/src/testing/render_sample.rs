use askama::Template;
use std::{fs, path::Path};

pub fn write_test_sandbox_output_for_dev(content: &str) {
    let p = Path::new(env!("FLUSTER_IOS_ROOT"));
    let x = p.join("packages").join("rust").join("conundrum").join("test_output.html");

    fs::write(x, content).expect("Failed to write test output");
}
