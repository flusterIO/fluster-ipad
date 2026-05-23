use conundrum_config::ecosystem::project::project_config::ProjectConfig;

use crate::{environments::web::next::write_next_output, errors::ConundrumCliResult};

pub async fn compile_directory(config: &ProjectConfig) -> ConundrumCliResult<()> {
    let files = config.get_blog_summaries()?;
    println!("Files: {:#?}", files);
    write_next_output(files, config)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use conundrum_config::traits::config_file::ConfigFile;

    use crate::utils::get_workspace_root_duplicate::get_workspace_root;

    use super::*;

    #[tokio::test]
    async fn compiles_directory() {
        let root = get_workspace_root();
        let input_path = std::path::Path::new(&root).join("apps")
                                                    .join("website")
                                                    .join("cdrm_config.json")
                                                    .to_str()
                                                    .expect("Compiles path properly")
                                                    .to_string();
        let config =
            ProjectConfig::read(&Some(input_path)).expect("Compiles website directory without throwing an error.");
        compile_directory(&config).await.expect("Parses directory without throwing an error.");
    }
}
