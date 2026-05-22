use conundrum_config::ecosystem::project::project_config::ProjectConfig;

use crate::{environments::web::next::write_next_output, errors::ConundrumCliResult, models::config::CliConfig};

pub async fn compile_directory(config: &ProjectConfig) -> ConundrumCliResult<()> {
    let files = config.get_files();
    write_next_output(files, config)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use conundrum::lang::runtime::state::parse_state::ConundrumCompileTarget;

    use crate::{
        models::config::{ConundrumSourceConfig, SourceOutputConfig},
        utils::get_workspace_root_duplicate::get_workspace_root,
    };

    use super::*;

    #[tokio::test]
    async fn compiles_directory() {
        let root = get_workspace_root();
        let input = std::path::Path::new(&root).join("apps")
                                               .join("website")
                                               .join("cdrm")
                                               .to_str()
                                               .expect("Compiles path properly")
                                               .to_string();

        let output_path = std::path::Path::new(&root).join("apps")
                                                     .join("website")
                                                     .join("src")
                                                     .join("features")
                                                     .join("cdrm")
                                                     .join("cdrm.json")
                                                     .to_str()
                                                     .expect("Compiles path properly")
                                                     .to_string();

        let config = CliConfig { source:
                                     ConundrumSourceConfig { input,
                                                             output:
                                                                 SourceOutputConfig { path: output_path,
                                                                                      format:
                                                                                          ConundrumCompileTarget::Html } },
                                 ..Default::default() };
        compile_directory(&config).await.expect("Parses directory without throwing an error.");
    }
}
