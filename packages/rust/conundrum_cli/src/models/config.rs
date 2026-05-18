use std::{
    error::Error,
    path::{Path, PathBuf},
};

use config::{Config, Environment, File, FileStoredFormat, Format};
use conundrum::{
    ecosystem::glue::conundrum_web_types::conundrum_web_builder::ConundrumWebProjectBuilder,
    lang::runtime::{
        run_conundrum::ParseConundrumOptions,
        state::{conundrum_error::ConundrumError, parse_state::ConundrumCompileTarget},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::errors::{ConundrumCliError, ConundrumCliResult};

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct SourceOutputConfig {
    pub path: String,
    pub format: ConundrumCompileTarget,
}

impl Default for SourceOutputConfig {
    fn default() -> Self {
        Self { path: String::from("conundrum"),
               format: ConundrumCompileTarget::Html }
    }
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct ConundrumSourceConfig {
    pub input: String,
    pub output: SourceOutputConfig,
}

impl Default for ConundrumSourceConfig {
    fn default() -> Self {
        Self { input: String::from("cdrm"),
               output: SourceOutputConfig::default() }
    }
}

pub fn default_conundrum_opts() -> ParseConundrumOptions {
    ParseConundrumOptions::default()
}

/// ## Plans
/// Shocker... I've accomplished none of these yet. This is basically just
/// a placeholder until I get [Fluster](https://flusterapp.com) released and I'm in a better living
/// situation to where I can focus more time on the cli.
///
/// - [ ] Read from file
///   - [ ] yaml
///   - [ ] json
///   - [ ] Conundrum (a future goal, once the logic layer is in place)
/// - [ ] Accept config file path as parameter.
/// - [ ] Allow global and project specific configuration.
#[derive(Default, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CliConfig {
    #[serde(default = "default_conundrum_opts")]
    pub opts: ParseConundrumOptions,
    pub build_target: ConundrumWebProjectBuilder,
    pub source: ConundrumSourceConfig,
}

impl CliConfig {
    pub fn read(cfg_path: &Option<String>) -> ConundrumCliResult<Self> {
        let config_path =
            cfg_path.as_ref().cloned().map(|s| Ok(Path::new(&s).to_path_buf())).unwrap_or_else(|| {
                                                                                    let cwd =
                                                                      std::env::current_dir().map_err(|e| {
                                                                          eprintln!("Error: {:#?}", e);
                                                                          ConundrumCliError::FsError("cwd".to_string())
                                                                      })?;
                                                                                    Ok(cwd.join("cdrm_config.json"))
                                                                                })?;
        let config_data = std::fs::read_to_string(&config_path).map_err(|e| {
                              eprintln!("Error: {:#?}", e);
                              ConundrumCliError::FsError(config_path.to_str().unwrap_or_default().to_string())
                          })?;
        let config: CliConfig = serde_json::from_str(&config_data).map_err(|e| {
                                                                      eprintln!("Error: {:#?}", e);
                                                                      ConundrumCliError::ProjectConfigError(None)
                                                                  })?;
        Ok(config)
    }
}
