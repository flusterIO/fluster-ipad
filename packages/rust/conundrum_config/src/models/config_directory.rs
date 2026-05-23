use std::path::PathBuf;

use conundrum::lang::constants::file_names::PROJECT_CONFIG_FILE_NAME;
use resolve_path::PathResolveExt;
use serde::{Deserialize, Serialize};

use crate::{
    ecosystem::project::project_config::ProjectConfig,
    errors::config_error::{ConfigError, ConfigResult},
};

/// This represents the location where the project is located.
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectDirectory(pub PathBuf);

impl Default for ProjectDirectory {
    fn default() -> Self {
        Self(std::env::current_dir().expect("Conundrum cannot continue. We can't figure out what your current directory is."))
    }
}

impl ProjectDirectory {
    pub fn make_relative_to_project_root(&self, p: PathBuf) -> PathBuf {
        self.0.join(p)
    }

    pub fn write_default_project_config(&self) -> ConfigResult<()> {
        let cfg = ProjectConfig::default();
        let y = serde_json::to_string_pretty(&cfg).map_err(|e| {
                                                      log::error!("Serialization Error: {}", e);
                                                      ConfigError::SerializationError
                                                  })?;
        let z = format!("./{}.json", PROJECT_CONFIG_FILE_NAME);
        let fp = &z.resolve_in(self.0.clone());
        std::fs::write(fp, y).map_err(|_| ConfigError::FileWriteError)
    }
}
