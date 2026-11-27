use std::{env, fs, io, path::Path};

fn copy_dir(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            copy_dir(&source_path, &destination_path)?;
        } else {
            fs::copy(source_path, destination_path)?;
        }
    }

    Ok(())
}

fn main() {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").expect("Cargo sets CARGO_MANIFEST_DIR");
    let manifest_dir = Path::new(&manifest_dir);
    let vendored_assets = manifest_dir.join("vendor/web_assets/dist");
    let workspace_assets = manifest_dir.join("../conundrum_web_assets/dist");
    let source = if vendored_assets.is_dir() {
        vendored_assets
    } else {
        workspace_assets
    };

    if !source.is_dir() {
        panic!("Conundrum web assets are missing at {}", source.display());
    }

    let out_dir = env::var_os("OUT_DIR").expect("Cargo sets OUT_DIR");
    let destination = Path::new(&out_dir).join("web_assets");
    copy_dir(&source, &destination).expect("copy Conundrum web assets into OUT_DIR");

    println!("cargo:rerun-if-changed={}", source.display());
}
