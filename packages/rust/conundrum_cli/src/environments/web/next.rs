use conundrum::ecosystem::glue::conundrum_web_types::{
    builder_output::next::{BlogFileSummary, NextJsConundrumOutput},
    conundrum_web_builder::ConundrumWebProjectBuilder,
};
use conundrum_config::ecosystem::project::project_config::ProjectConfig;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::path::Path;

use crate::{
    errors::{ConundrumCliError, ConundrumCliResult},
    models::conundrum_file::ConundrumFile,
};

pub fn write_next_output(files: Vec<BlogFileSummary>, opts: &ProjectConfig) -> ConundrumCliResult<()> {
    let cfg = NextJsConundrumOutput { files };

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
                   }.ok_or(ConundrumCliError::ProjectConfigError(None)),
        None => {
            if opts.build_target == ConundrumWebProjectBuilder::Next {
                std::fs::write(p, "{{}}").map_err(|e| {
                                             eprintln!("Error: {:#?}", e);
                                             ConundrumCliError::ProjectConfigError(None)
                                         })?;
                p.to_str().map(String::from).ok_or(ConundrumCliError::ProjectConfigError(None))
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
