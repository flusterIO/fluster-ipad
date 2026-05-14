use crate::{commands::parse_directory_to_directory::parse_directory_to_directory, models::config::CliConfig};
use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;

/// This currently does not cache or do anything incrementally...
pub async fn watch_directory(config: &CliConfig) -> Result<()> {
    // 1. Create a channel to receive events
    let (tx, rx) = std::sync::mpsc::channel();

    // 2. Automatically select the best implementation for your platform
    let mut watcher: RecommendedWatcher = notify::recommended_watcher(tx)?;

    // 3. Add a path to be watched (current directory)
    watcher.watch(Path::new(&config.source.input), RecursiveMode::Recursive)?;

    // 4. Process events
    for res in rx {
        match res {
            Ok(event) => {
                println!("Event: {:#?}", event.source());
                let _ = parse_directory_to_directory(&config.source.input, &config.source.output.path, &config).await;
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}
