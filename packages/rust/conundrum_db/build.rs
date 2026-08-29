use std::{env, process::Command};

fn main() {
    let workspace = env::var("FLUSTER_IOS_ROOT").unwrap();

    // let status = Command::new("cargo").current_dir(&workspace)
    //                                   .args(["run", "--package", "conundrum_docgen", "write-db-models"])
    //                                   .status()
    //                                   .expect("failed to execute docgen");

    // assert!(status.success(), "docgen failed");
}
