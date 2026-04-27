/// Had to duplicate this to avoid a dependency on the Fluster core utilites
/// crate.
pub fn get_workspace_root() -> String {
    let x = std::env!("CARGO_MANIFEST_DIR");
    let p = std::path::Path::new(x).parent().unwrap().parent().unwrap().parent().unwrap();
    p.to_str().unwrap().to_string()
}
