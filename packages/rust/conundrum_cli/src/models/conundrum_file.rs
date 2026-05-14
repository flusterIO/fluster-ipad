use std::path::{Path, PathBuf};

use conundrum::{
    lang::runtime::{run_conundrum::run_conundrum, state::parse_state::ConundrumCompileTarget},
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};

use crate::{
    errors::{ConundrumCliError, ConundrumCliResult},
    models::config::CliConfig,
};

pub struct ConundrumFile {
    pub absolute_path: PathBuf,
}

impl ConundrumFile {
    pub fn parse(&self, opts: &CliConfig) -> ConundrumCliResult<MdxParsingResult> {
        let content = std::fs::read_to_string(self.absolute_path.clone()).map_err(|_| {
                                                                             let s = self.absolute_path
                                                                                         .to_str()
                                                                                         .map(String::from)
                                                                                         .unwrap_or_default();
                                                                             ConundrumCliError::FsError(s)
                                                                         })?;
        let new_opts = opts.0.duplicate_with_new_content(content);
        let x = run_conundrum(new_opts).map_err(ConundrumCliError::ConundrumError)?;
        Ok(x)
    }

    pub fn parse_to_relative_directory(&self,
                                       input_dir: &str,
                                       output_dir: &str,
                                       opts: &CliConfig)
                                       -> ConundrumCliResult<()> {
        let output = self.parse(opts)?;
        let p = self.absolute_path.strip_prefix(input_dir).map_err(|_| {
            ConundrumCliError::FileNotChildOfDir(self.absolute_path.to_str().map(String::from).unwrap_or_default(), input_dir.to_string())
        })?;
        let mut output_path = Path::new(output_dir).join(p);
        output_path.set_extension(opts.0.target.to_file_ext());
        println!("Path: {:#?}", output_path);
        std::fs::write(output_path.clone(), output.content).map_err(|_| {
                                                               ConundrumCliError::FsError(output_path.to_str()
                                                                                             .map(String::from)
                                                                                             .unwrap_or_default())
                                                           })?;
        Ok(())
    }
}
