use std::path::PathBuf;

use crate::errors::config_error::{ConfigError, ConfigResult};

pub fn cwd() -> ConfigResult<PathBuf> {
    std::env::current_dir().map_err(|e| {
                               log::error!("Error: {}", e);
                               ConfigError::FailToFindProjectRoot
                           })
}
