use conundrum::ecosystem::glue::conundrum_web_types::{
    builder_output::next::{NextJsConundrumOutput, NextjsFileSummary},
    conundrum_web_builder::ConundrumWebProjectBuilder,
};
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::path::Path;

use crate::{
    errors::{ConundrumCliError, ConundrumCliResult},
    models::{config::CliConfig, conundrum_file::ConundrumFile},
};

pub fn write_next_output(written_files: Vec<ConundrumFile>, opts: &CliConfig) -> ConundrumCliResult<()> {
    let files: ConundrumCliResult<Vec<NextjsFileSummary>> =
        written_files.par_iter()
                     .map(|f| {
                         let relative_path = f.get_relative_nested_path(opts.source.input.clone())?;
                         Ok(NextjsFileSummary { html: f.results.content.clone(),
                                                relative_path,
                                                front_matter: f.results.front_matter.clone(),
                                                keywords: Vec::new() })
                     })
                     .collect();

    match files {
        Ok(f) => {
            let cfg = NextJsConundrumOutput { files: f };

            let s = serde_json::to_string(&cfg).map_err(|e| {
                                                   eprintln!("Error: {:#?}", e);
                                                   ConundrumCliError::SerializationError
                                               })?;

            let p = Path::new(&opts.source.output.path);
            let meta = p.metadata().ok();

            let x = match meta {
                Some(m) => match m.is_dir() {
                               true => p.join("cdrm_output.json").to_str().map(String::from),
                               false => p.to_str().map(String::from),
                           }.ok_or_else(|| ConundrumCliError::ProjectConfigError),
                None => {
                    if opts.build_target == ConundrumWebProjectBuilder::Next {
                        std::fs::write(p, "{{}}").map_err(|e| {
                                                     eprintln!("Error: {:#?}", e);
                                                     ConundrumCliError::ProjectConfigError
                                                 })?;
                        p.to_str().map(String::from).ok_or_else(|| ConundrumCliError::ProjectConfigError)
                    } else {
                        Err(ConundrumCliError::NotImplemented)
                    }
                }
            }?;

            std::fs::write(x, s).map_err(|e| {
                                    eprintln!("Error: {:#?}", e);
                                    ConundrumCliError::FsError(opts.source.output.path.clone())
                                })?;

            Ok(())
        }
        Err(e) => Err(e),
    }
}
