use crate::{
    errors::{ConundrumCliError, ConundrumCliResult},
    models::{config::CliConfig, conundrum_file::ConundrumFile},
};

pub async fn parse_directory_to_directory(input_dir: &str,
                                          output_dir: &str,
                                          opts: &CliConfig)
                                          -> ConundrumCliResult<()> {
    let files = std::fs::read_dir(input_dir)?;
    for f in files {
        let res = f.map_err(|_| ConundrumCliError::FsError("Unknown".to_string()))?;
        if let Ok(meta) = res.metadata() {
            let _path = res.path();
            if _path.extension().is_some_and(|s| s == "cdrm") {
                let f = ConundrumFile { absolute_path: _path };
                f.parse_to_relative_directory(input_dir, output_dir, opts)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use conundrum::lang::runtime::run_conundrum::ParseConundrumOptions;

    use super::*;

    #[tokio::test]
    async fn parses_directory_to_directory() {
        let res = parse_directory_to_directory("/Users/bigsexy/Desktop/swift/Fluster/apps/website/cdrm/",
                                               "/Users/bigsexy/Desktop/swift/Fluster/apps/website/public/cdrm/", &CliConfig(ParseConundrumOptions::default())).await.expect("Parses directory without throwing an error.");
    }
}
