use std::path::PathBuf;

pub trait DefaultWithSourcePath {
    fn default(src: PathBuf) -> Self;
}
