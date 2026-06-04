use fluster_core_utilities::test_utilities::get_workspace_root::get_workspace_root;

/// Accepts a path relative to the **monorepo root**.
pub fn get_documentation_content(relative_path: &str) -> String {
    let root = get_workspace_root();
    let p = std::path::Path::new(&root).join(relative_path);

    std::fs::read_to_string(p).inspect_err(|e| {
                                  eprintln!("Could not read documentation at the `{}` path.", relative_path.to_string())
                              })
                              .unwrap()
}
