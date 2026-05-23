use crate::{
    environments::web::next::write_next_output,
    errors::{ConundrumCliError, ConundrumCliResult},
};
use conundrum_config::ecosystem::project::project_config::ProjectConfig;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::Path;

pub async fn watch_directory(config: &ProjectConfig) -> ConundrumCliResult<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher: RecommendedWatcher = notify::recommended_watcher(tx).map_err(|e| {
                                              println!("Error: {:#?}", e);
                                              ConundrumCliError::FsError("unknown".to_string())
                                          })?;

    watcher.watch(config.source.input.resolve().as_path(), RecursiveMode::Recursive).map_err(|e| {
                                              println!("Error: {:#?}", e);
                                              ConundrumCliError::FsError("unknown".to_string())
    })?;

    for res in rx {
        match res {
            Ok(event) => {
                println!("Event: {:#?}", event.source());
                let files = config.get_files()?;
                write_next_output(files.par_iter().map(|item| item.to_blog_summary()).collect(), config)?;
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
