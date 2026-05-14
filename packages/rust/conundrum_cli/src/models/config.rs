use conundrum::lang::runtime::{run_conundrum::ParseConundrumOptions, state::parse_state::ConundrumCompileTarget};
use serde::{Deserialize, Serialize};

use crate::errors::{ConundrumCliError, ConundrumCliResult};

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
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
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct CliConfig {
    #[serde(default = "default_conundrum_opts")]
    pub opts: ParseConundrumOptions,
    pub source: ConundrumSourceConfig,
}

impl CliConfig {
    pub fn read(relative_path: &Option<String>) -> ConundrumCliResult<Self> {
        let content =
            std::fs::read_to_string(relative_path.clone().unwrap_or("./cdrm.config.json".to_string())).map_err(|e| {
                println!("Error: {:#?}", e);
                ConundrumCliError::ProjectConfigError
            })?;
        let config: CliConfig = serde_json::from_str(content.as_str()).map_err(|e| {
                                                                          println!("Error: {:#?}", e);
                                                                          ConundrumCliError::ProjectConfigError
                                                                      })?;
        Ok(config)
    }
}
