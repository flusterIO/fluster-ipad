use std::{path::PathBuf, str::FromStr};

use crate::errors::ConundrumCliResult;

pub struct ProjectPath(pub PathBuf);

impl ProjectPath {
    pub fn from_string(input: &str, config_dir_path: &str) -> Self {
        let p = std::path::Path::new(input);
        if p.is_absolute() {
            ProjectPath(p.to_path_buf())
        } else {
            let r = std::path::Path::new(config_dir_path).join(p);
            ProjectPath(r)
        }
    }
}
