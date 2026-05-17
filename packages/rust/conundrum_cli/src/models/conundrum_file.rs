use std::path::{Path, PathBuf};

use conundrum::{
    lang::runtime::run_conundrum::run_conundrum, output::parsing_result::mdx_parsing_result::MdxParsingResult,
};

use crate::{
    errors::{ConundrumCliError, ConundrumCliResult},
    models::config::CliConfig,
};

#[derive(Debug)]
pub struct ConundrumFile {
    pub absolute_path: PathBuf,
    pub results: MdxParsingResult,
}

impl ConundrumFile {
    /// This will throw an error if the path is not a child of the parent.
    pub fn get_relative_nested_path(&self, dir_path: String) -> ConundrumCliResult<String> {
        let r = self.absolute_path.strip_prefix(dir_path).map_err(|e| {
                                                              eprintln!("Error: {:#?}", e);
                                                              ConundrumCliError::FsError(self.absolute_path
                                                                                             .to_str()
                                                                                             .map(String::from)
                                                                                             .unwrap_or_default())
                                                          })?;
        match r.to_str() {
            Some(x) => Ok(x.to_string()),
            None => {
                eprintln!("An empty nested path is not valid");
                Err(ConundrumCliError::FsError(self.absolute_path.to_str().map(String::from).unwrap_or_default()))
            }
        }
    }

    pub fn write_to_relative_directory(&self,
                                       input_dir: &str,
                                       output_dir: &str,
                                       opts: &CliConfig)
                                       -> ConundrumCliResult<()> {
        let p = self.absolute_path.strip_prefix(input_dir).map_err(|_| {
            ConundrumCliError::FileNotChildOfDir(self.absolute_path.to_str().map(String::from).unwrap_or_default(), input_dir.to_string())
        })?;
        let mut output_path = Path::new(output_dir).join(p);
        output_path.set_extension(opts.opts.target.to_file_ext());
        std::fs::write(output_path.clone(), self.results.content.clone()).map_err(|e| {
            eprintln!("Error: {:#?}", e);
            ConundrumCliError::FsError(output_path.to_str().map(String::from).unwrap_or_default())
        })?;
        Ok(())
    }

    pub fn from_absolute_path(path: &str, opts: &CliConfig) -> ConundrumCliResult<ConundrumFile> {
        let content = std::fs::read_to_string(path).map_err(|_| ConundrumCliError::FsError(path.to_string()))?;
        let new_opts = opts.opts.duplicate_with_new_content(content);
        let x = run_conundrum(new_opts).map_err(ConundrumCliError::ConundrumError)?;
        Ok(ConundrumFile { absolute_path: path.into(),
                           results: x.clone() })
    }
}
