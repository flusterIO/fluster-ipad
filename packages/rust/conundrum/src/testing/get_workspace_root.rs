use std::path::Path;

pub fn get_workspace_root() -> String {
    let x = std::env!("CARGO_MANIFEST_DIR");
    let p = Path::new(x).parent().unwrap().parent().unwrap().parent().unwrap();
    p.to_str().unwrap().to_string()
}
