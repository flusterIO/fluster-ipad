use std::path::{Path, PathBuf};

use conundrum::{
    lang::runtime::{
        run_conundrum::{run_conundrum, ParseConundrumOptions},
        state::conundrum_error_variant::ConundrumResult,
    },
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};
use conundrum_config::ecosystem::project::project_config::ProjectConfig;

use crate::{
    errors::conundrum_fs_error::{ConundrumFSError, ConundrumFSResult},
    traits::parsable_file::ParsableFile,
};

#[derive(Debug)]
pub struct ConundrumFile {
    pub absolute_path: PathBuf,
    pub results: MdxParsingResult,
}

impl ParsableFile for ConundrumFile {
    fn as_file_content(&self) -> conundrum::lang::runtime::state::conundrum_error_variant::ConundrumResult<String> {
        Ok(self.results.content.clone())
    }

    fn parse_string(content: String, absolute_path: &PathBuf, opts: &ParseConundrumOptions) -> ConundrumResult<Self>
        where Self: Sized {
        let new_opts = opts.duplicate_with_new_content(content);
        run_conundrum(new_opts).map(|results| ConundrumFile { results,
                                                              absolute_path: absolute_path.clone() })
    }
}

// impl FileVisitor for ConundrumFile {
// }

impl ConundrumFile {
    /// This will throw an error if the path is not a child of the parent.
    pub fn get_relative_nested_path(&self, dir_path: String) -> ConundrumFSResult<String> {
        let r = self.absolute_path.strip_prefix(dir_path).map_err(|e| {
                                                              eprintln!("Error: {:#?}", e);
                                                              ConundrumFSError::FsError(self.absolute_path
                                                                                            .to_str()
                                                                                            .map(String::from)
                                                                                            .unwrap_or_default())
                                                          })?;
        match r.to_str() {
            Some(x) => Ok(x.to_string()),
            None => {
                eprintln!("An empty nested path is not valid");
                Err(ConundrumFSError::FsError(self.absolute_path.to_str().map(String::from).unwrap_or_default()))
            }
        }
    }

    pub fn write_to_relative_directory(&self,
                                       input_dir: &str,
                                       output_dir: &str,
                                       opts: &ProjectConfig)
                                       -> ConundrumFSResult<()> {
        let p = self.absolute_path.strip_prefix(input_dir).map_err(|_| {
                                                               ConundrumFSError::FileNotChildOfDir(self.absolute_path
                                                                                                   .to_str()
                                                                                                   .map(String::from)
                                                                                                   .unwrap_or_default(),
                                                                                               input_dir.to_string())
                                                           })?;
        let mut output_path = Path::new(output_dir).join(p);
        output_path.set_extension(opts.opts.target.to_file_ext());
        std::fs::write(output_path.clone(), self.results.content.clone()).map_err(|e| {
            eprintln!("Error: {:#?}", e);
            ConundrumFSError::FsError(output_path.to_str().map(String::from).unwrap_or_default())
        })?;
        Ok(())
    }

    pub fn from_absolute_path(path: &str, opts: &ProjectConfig) -> ConundrumFSResult<ConundrumFile> {
        let content = std::fs::read_to_string(path).map_err(|_| ConundrumFSError::FsError(path.to_string()))?;
        let new_opts = opts.opts.duplicate_with_new_content(content);
        let x = run_conundrum(new_opts).map_err(ConundrumFSError::ConundrumError)?;
        Ok(ConundrumFile { absolute_path: path.into(),
                           results: x.clone() })
    }
}
