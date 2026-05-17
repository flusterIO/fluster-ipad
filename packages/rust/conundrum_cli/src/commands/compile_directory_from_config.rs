use crate::{
    commands::parse_directory_to_directory::get_directory_conundrum_files, environments::web::next::write_next_output,
    errors::ConundrumCliResult, models::config::CliConfig,
};

pub async fn compile_directory(config: &CliConfig) -> ConundrumCliResult<()> {
    let files = get_directory_conundrum_files(&config.source.input, config).await?;
    println!("Files: {:#?}", files);
    write_next_output(files, config)?;
    Ok(())
}
