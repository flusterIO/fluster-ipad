use std::{path::PathBuf, sync::Arc};

use conundrum::{
    ecosystem::glue::conundrum_web_types::conundrum_web_builder::ConundrumWebProjectBuilder,
    lang::{
        constants::{file_names::PROJECT_CONFIG_FILE_NAME, file_types::ParsableFileType},
        runtime::{
            run_conundrum::{ParseConundrumOptions, run_conundrum},
            state::{conundrum_error_variant::ConundrumResult, parse_state::ConundrumCompileTarget},
        },
    },
};
use figment::{
    Figment,
    providers::{Format, Toml},
};
use ignore::{DirEntry, Error, WalkState};
use parking_lot::Mutex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    ecosystem::{project::project_file_description::ProjectFileDescription, shared::ignore::IgnoreConfig},
    errors::config_error::{ConfigError, ConfigResult},
    traits::{
        config_file::ConfigFile,
        file_collection_producer::{FileCollectionRequest, FileCollectionVisitor},
    },
};

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct SourceOutputConfig {
    pub path: String,
    pub format: ConundrumCompileTarget,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct ConundrumSourceConfig {
    pub input: String,
    pub output: SourceOutputConfig,
}

pub fn default_conundrum_opts() -> ParseConundrumOptions {
    ParseConundrumOptions::default()
}

/// ## Plans
/// Shocker... I've accomplished none of these yet. This is basically just
/// a placeholder until I get [Fluster](https://flusterapp.com) released and I'm in a better living
/// situation to where I can focus more time on the cli.
///
/// - [ ] Allow relative paths being while being executable from anywhere... my
///   *%#@&* pet peeve.
/// - [ ] Read from file
///   - [x] yaml
///   - [x] json
///   - [ ] Conundrum (a future goal, once the logic layer is in place)
/// - [ ] Allow environment variable overrides (probably going to use figment
///   for this) for the
/// easy relative path support.
/// - [ ] Accept config file path as parameter.
/// - [ ] Allow global and project specific configuration.
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct ProjectConfig {
    pub name: String,
    /// The description, as conundrum content.
    /// Be smart about what you use here though, because
    /// this will be inserted into places like a generated website's
    /// description. The parser will try to remove ridiculus things like
    /// images, but it might not catch everything.
    pub desc: String,
    #[serde(default = "default_conundrum_opts")]
    pub opts: ParseConundrumOptions,
    pub build_target: ConundrumWebProjectBuilder,
    pub source: ConundrumSourceConfig,
    pub root: PathBuf,
    pub ignore: IgnoreConfig,
}

pub fn match_any_conundrum_file(p: DirEntry) -> bool {
    if let Some(file_extension) = p.path().extension() {
        [ParsableFileType::Cdrm.to_string()].contains(&file_extension.to_str().map(String::from).unwrap_or_default())
    } else {
        false
    }
}

impl ProjectConfig {
    pub fn get_files(&self) -> ConfigResult<Vec<ProjectFileDescription>> {
        let mut items: Arc<Mutex<Vec<ProjectFileDescription>>> = Arc::new(Mutex::new(Vec::new()));
        // let root_path = self.
        let req = FileCollectionRequest { root: self.root.clone(),
                                          respect_gitignore: self.ignore.respect_gitignore,
                                          should_parse: match_any_conundrum_file,
                                          callback: || {
                                              Box::new(|entry| {
                                                  if let Ok(res) = entry {
                                                      if let Ok(file_content) = std::fs::read_to_string(res.path()) {
                                                          if let Ok(parse_response) = run_conundrum(self.opts.duplicate_with_new_content(file_content)) {
                                                                                                                        let mut locked_items = items.lock_arc();
                                                          locked_items.push(ProjectFileDescription { 
                                                              input_path: res.path().to_path_buf(),
                                                              root_path: ,
                                                          results: parse_response.clone()
                                                          });

                                                          }
                                                      }
                                                  };
                                                  WalkState::Continue
                                              })
                                          } };

        if let Err(visit_err) = self.visit_files(req) {
            eprintln!("Error: {}", visit_err);
        }
        let returned_items = items.lock();
        Ok(returned_items.to_vec())
    }
}

impl<'a, T, J> FileCollectionVisitor<'a, T, J> for ProjectConfig
    where T: FnMut() -> Box<dyn FnMut(Result<DirEntry, Error>) -> WalkState + Send + 'a>,
          J: Fn(DirEntry) -> bool
{
    fn visit_files(&self, req: FileCollectionRequest<'a, T, J>) -> ConundrumResult<()> {
        let w =
ignore::WalkBuilder::new(req.root).
add_custom_ignore_filename(conundrum::lang::constants::file_names::CDRM_IGNORE_FILE_NAME)
                                          .git_ignore(req.respect_gitignore)
                                          .build_parallel();
        w.run(req.callback);
        Ok(())
    }
}

impl ConfigFile for ProjectConfig {
    fn filename() -> &'static str {
        PROJECT_CONFIG_FILE_NAME
    }

    fn figment() -> Figment {
        let file_name = Self::filename();
        Figment::new().merge(Toml::file(file_name))
                      .merge(figment::providers::Json::file(file_name))
                      .merge(figment::providers::YamlExtended::file(file_name))
    }

    fn read<'a>() -> crate::errors::config_error::ConfigResult<Self>
        where Self: Sized + Deserialize<'a> {
        let f: Self = Self::figment().extract().map_err(|e| {
                                                    std::eprintln!("Error: {}", e);
                                                    ConfigError::OhShit
                                                })?;
        Ok(f)
    }
}
