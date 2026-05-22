use conundrum::{
    ecosystem::glue::conundrum_web_types::conundrum_web_builder::ConundrumWebProjectBuilder,
    lang::runtime::{
        run_conundrum::ParseConundrumOptions,
        state::{parse_state::ConundrumCompileTarget},
    },
};
use figment::{
    Figment,
    providers::{Format, Json, Toml, YamlExtended, Env},
};
use ignore::{DirEntry, Error, WalkBuilder, WalkState};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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

#[derive(Default, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CliConfig {
    #[serde(default = "default_conundrum_opts")]
    pub opts: ParseConundrumOptions,
    pub build_target: ConundrumWebProjectBuilder,
    pub source: ConundrumSourceConfig,
}

impl CliConfig {
    fn figment() -> Figment {
        Figment::new()
            .merge(
                Toml::file("cdrm_config.toml").nested()
                )
            .merge(
                Json::file("cdrm_config.json").nested()
                )
            .merge(
YamlExtended::file("cdrm_config.yaml")
            )
        
    }

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

    // pub fn visit_files<'a, F>(visitor: F)
    //     where F: FnMut() -> Box<dyn FnMut(Result<DirEntry, Error>) -> WalkState +
    // Send + 'a> {     let r =
    // WalkBuilder::new(path).add_custom_ignore_filename("cdrm_ignore").
    // build_parallel().run(visitor); }
}
