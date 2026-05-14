use crate::{
    errors::{ConundrumCliError, ConundrumCliResult},
    models::{config::CliConfig, conundrum_file::ConundrumFile},
};

pub async fn get_directory_conundrum_files(dir_path: &str, opts: &CliConfig) -> ConundrumCliResult<Vec<ConundrumFile>> {
    let files = std::fs::read_dir(dir_path).map_err(|e| {
                                               eprintln!("Error: {:#?}", e);
                                               ConundrumCliError::FsError(dir_path.to_string())
                                           })?;
    let mut cdrm_files = Vec::new();
    for f in files {
        let res = f.map_err(|_| ConundrumCliError::FsError("Unknown".to_string()))?;
        let _path = res.path();
        if _path.extension().is_some_and(|s| s == "cdrm" || s == "mdx") {
            match ConundrumFile::from_absolute_path(_path.to_str().unwrap_or_default(), opts) {
                Ok(f) => {
                    cdrm_files.push(f);
                }
                Err(e) => {
                    eprintln!("Found an error in a conundrum file: {:#?}", e);
                }
            }
        }
    }
    Ok(cdrm_files)
}

pub async fn parse_directory_to_directory(input_dir: &str,
                                          output_dir: &str,
                                          opts: &CliConfig)
                                          -> ConundrumCliResult<Vec<ConundrumFile>> {
    let files = std::fs::read_dir(input_dir).map_err(|e| {
                                                eprintln!("Error: {:#?}", e);
                                                ConundrumCliError::FsError(input_dir.to_string())
                                            })?;
    let mut cdrm_files = Vec::new();
    for f in files {
        let res = f.map_err(|_| ConundrumCliError::FsError("Unknown".to_string()))?;
        let _path = res.path();
        if _path.extension().is_some_and(|s| s == "cdrm" || s == "mdx") {
            match ConundrumFile::from_absolute_path(_path.to_str().unwrap_or_default(), opts) {
                Ok(f) => {
                    f.write_to_relative_directory(input_dir, output_dir, opts)?;
                    cdrm_files.push(f);
                }
                Err(e) => {
                    eprintln!("Found an error in a conundrum file: {:#?}", e);
                }
            }
        }
    }
    Ok(cdrm_files)
}
