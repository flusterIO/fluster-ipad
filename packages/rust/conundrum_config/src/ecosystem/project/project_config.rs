use std::{env, path::PathBuf, sync::Arc}; 
use rayon::prelude::*;

use conundrum::{
    ecosystem::glue::conundrum_web_types::{builder_output::next::BlogFileSummary, conundrum_web_builder::ConundrumWebProjectBuilder},
    lang::{
        constants::{file_names::PROJECT_CONFIG_FILE_NAME, file_types::ParsableFileType},
        runtime::{
            run_conundrum::{ParseConundrumOptions, run_conundrum},
            state::{conundrum_error_variant::ConundrumResult, parse_state::ConundrumCompileTarget},
        },
    },
};
use ignore::{DirEntry, Error, WalkState};
use parking_lot::Mutex;
use schemars::{JsonSchema};
use serde::{Deserialize, Serialize};
use config::{Config, Environment, File};

use crate::{
    ecosystem::{project::project_file_description::ProjectFileDescription, shared::ignore::IgnoreConfig}, errors::config_error::{ ConfigError, ConfigResult}, models::config_path::ConfigPath, traits::{
        config_file::ConfigFile,
        file_collection_producer::{FileCollectionRequest, FileCollectionVisitor},
    }, utils::cwd::cwd
};

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
#[serde(default)]
pub struct SourceOutputConfig {
    pub path: String,
    pub format: ConundrumCompileTarget,
}

impl Default for SourceOutputConfig {
    fn default() -> Self {
        Self { path: "./cdrm_output".to_string(), format: Default::default() }
    }
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
#[serde(default)]
pub struct ConundrumSourceConfig {
    pub input: ConfigPath,
    pub output: SourceOutputConfig,
}

impl Default for ConundrumSourceConfig {
    fn default() -> Self {
        Self { input: ConfigPath::from_str_or_panic("./cdrm"), output: Default::default() }
    }
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
#[serde(default)]
pub struct ProjectConfig {
    pub name: String,
    /// The description, as conundrum content.
    /// Be smart about what you use here though, because
    /// this will be inserted into places like a generated website's
    /// description. The parser will try to remove ridiculus things like
    /// images, but it might not catch everything.
    pub desc: String,
    pub opts: ParseConundrumOptions,
    pub build_target: ConundrumWebProjectBuilder,
    pub source: ConundrumSourceConfig,
    #[schemars(skip)]
    #[serde(skip)]
    pub root: PathBuf,
    pub ignore: IgnoreConfig,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self { name: "Conundrum".to_string(),
        desc: "Built with the Conundrum compiler. See Flusterapp.com for more information.".to_string(),
        opts: Default::default(),
        build_target: Default::default(),
        source: Default::default(),
        root: Default::default(),
        ignore: Default::default()
        }
    }
}

pub fn match_any_conundrum_file(p: DirEntry) -> bool {
    if let Some(file_extension) = p.path().extension() {
        [ParsableFileType::Cdrm.to_string()].contains(&file_extension.to_str().map(String::from).unwrap_or_default())
    } else {
        false
    }
}

impl ProjectConfig {
    pub fn get_blog_summaries(&self) -> ConfigResult<Vec<BlogFileSummary>>{
        let r = self.get_files()?.par_iter().map(|item| {
            item.to_blog_summary()
        }).collect::<Vec<BlogFileSummary>>();
        Ok(r)
    }
    pub fn get_files(&self) -> ConfigResult<Vec<ProjectFileDescription>> {
        let items: Arc<Mutex<Vec<ProjectFileDescription>>> = Arc::new(Mutex::new(Vec::new()));
        let root_path = self.root.clone();
        println!("Root Path: {:#?}", root_path);
        let req = FileCollectionRequest { root: root_path.clone(),
                                          respect_gitignore: self.ignore.respect_gitignore,
                                          should_parse: match_any_conundrum_file,
                                          callback: || {
                                              Box::new(|entry| {
                                                  if let Ok(res) = entry {
                                                      let f = res.path().to_path_buf();
                                                      let file_extension = f.extension().map(|s| s.to_str()).flatten();
                                                      println!("File Extension: {:#?}", file_extension);
                                                      if file_extension.is_some_and(ParsableFileType::extension_is_conundrum_file) {

                                                      if let Ok(file_content) = std::fs::read_to_string(f.clone()) {
                                                          if let Ok(parse_response) = run_conundrum(self.opts.duplicate_with_new_content(file_content)) {
                                                                                                                        let mut locked_items = items.lock_arc();
                                                          locked_items.push(ProjectFileDescription { 
                                                              input_path: res.path().to_path_buf(),
                                                              root_path: root_path.clone(),
                                                          results: parse_response.clone()
                                                          });
                                                          }
                                                      }
                                                      WalkState::Continue
                                                  } else {
                                                      WalkState::Continue
                                                  }
                                                      } else {
                                                      WalkState::Skip
                                                      }
                                              })
                                          } };

        if let Err(visit_err) = self.visit_files(req) {
            eprintln!("File Error: {}", visit_err);
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


    fn read(config_path_override: &Option<String>) -> crate::errors::config_error::ConfigResult<Self>
        where Self: Sized {
        if let Some(cp) = config_path_override {
            if let Err(err) = std::env::set_current_dir(cp) {
                log::error!("Conundrum could not update the working directory. I hate myself for even implementing it this way anyways, so it's probably for the better. It looks like you'll have to 'cd' yourself. Here's the actual error: {}", err);
            }
        }
        let c = Config::builder()
            .add_source(File::with_name(PROJECT_CONFIG_FILE_NAME).required(true))
            .add_source(Environment::with_prefix("CDRM"))
            .build()
            .map_err(|e| {
                log::error!("Config Error: {}", e);
                ConfigError::OhShit
            })?;

        let mut s: Self = c.try_deserialize().map_err(|e| {
            log::error!("Config Error: {}", e);
            ConfigError::SerializationError
        })?;

        s.root = cwd()?;

        Ok(s)
    }

}
