use crate::{
    commands::parse_directory_to_directory::get_directory_conundrum_files,
    environments::web::next::write_next_output,
    errors::{ConundrumCliError, ConundrumCliResult},
    models::config::CliConfig,
};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

pub async fn watch_directory(config: &CliConfig) -> ConundrumCliResult<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher: RecommendedWatcher = notify::recommended_watcher(tx).map_err(|e| {
                                              println!("Error: {:#?}", e);
                                              ConundrumCliError::FsError("unknown".to_string())
                                          })?;

    watcher.watch(Path::new(&config.source.input), RecursiveMode::Recursive).map_err(|e| {
                                              println!("Error: {:#?}", e);
                                              ConundrumCliError::FsError("unknown".to_string())
    })?;

    for res in rx {
        match res {
            Ok(event) => {
                println!("Event: {:#?}", event.source());
                // TODO: This will currently only work for Next.js, and pretty much for a setup
                // like I have for the Fluster website. I'm going to get around to
                // implementing the different build environments (vite, node,
                // etc.) when I have a place with wifi, because that seems kind of wifi
                // intensive and I can't give up that precious library time right now.
                let files = get_directory_conundrum_files(&config.source.input, config).await?;
                write_next_output(files, config)?;
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}
