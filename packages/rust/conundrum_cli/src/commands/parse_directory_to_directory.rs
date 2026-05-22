use conundrum_config::ecosystem::project::project_config::ProjectConfig;
use ignore::{WalkBuilder, WalkParallel};

use crate::{
    errors::{ConundrumCliError, ConundrumCliResult},
    models::conundrum_file::ConundrumFile,
};

pub async fn parse_directory_to_directory(input_dir: &str,
                                          output_dir: &str,
                                          cfg: &ProjectConfig)
                                          -> ConundrumCliResult<Vec<ConundrumFile>> {
    let mut cdrm_files: Vec<ConundrumFile> = Vec::new();

    // opts.on_green

    Ok(cdrm_files)
}
