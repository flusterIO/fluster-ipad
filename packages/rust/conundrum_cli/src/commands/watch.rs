use std::{
    collections::{BTreeMap, HashSet},
    path::PathBuf,
};

use crate::{
    environments::web::next::write_next_output,
    errors::{ConundrumCliError, ConundrumCliResult},
};
use conundrum_config::ecosystem::project::{
    project_config::ProjectConfig, project_file_description::ProjectFileDescription,
};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn write_output(cache: &BTreeMap<PathBuf, ProjectFileDescription>, config: &ProjectConfig) -> ConundrumCliResult<()> {
    let files: Vec<&ProjectFileDescription> = cache.values().collect();
    write_next_output(files.par_iter().map(|item| item.to_blog_summary()).collect(), config)
}

/// Watches the project's input directory. The whole directory is parsed once up front; after that, each
/// change event only re-parses the file(s) that changed.
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

    let mut cache: BTreeMap<PathBuf, ProjectFileDescription> =
        config.get_files()?.into_iter().map(|f| (f.input_path.clone(), f)).collect();
    write_output(&cache, config)?;

    for res in &rx {
        let mut changed: HashSet<PathBuf> = HashSet::new();
        let mut collect = |res: notify::Result<notify::Event>| match res {
            Ok(event) => {
                // Reads and other non-mutating events can't change the output.
                if matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) | EventKind::Any) {
                    changed.extend(event.paths);
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        };
        collect(res);
        // Editors usually fire a burst of events per save; handle them as one batch.
        while let Ok(more) = rx.try_recv() {
            collect(more);
        }
        if changed.is_empty() {
            continue;
        }

        let mut needs_full_rescan = false;
        let mut dirty = false;
        for path in changed {
            let known = cache.contains_key(&path);
            if path.is_file() {
                if known {
                    // Keep the previous version if the new content fails to parse.
                    if let Some(updated) = config.parse_file(&path) {
                        cache.insert(path, updated);
                        dirty = true;
                    }
                } else {
                    // A path we haven't seen: it might be new, or it might be ignored by .gitignore / the
                    // cdrm ignore file. A full rescan is the only way to apply those rules correctly.
                    needs_full_rescan = true;
                }
            } else if known {
                cache.remove(&path);
                dirty = true;
            } else if !path.exists() {
                // A removed directory takes its children with it.
                let before = cache.len();
                cache.retain(|p, _| !p.starts_with(&path));
                dirty |= cache.len() != before;
            }
        }

        if needs_full_rescan {
            cache = config.get_files()?.into_iter().map(|f| (f.input_path.clone(), f)).collect();
            dirty = true;
        }
        if dirty {
            write_output(&cache, config)?;
        }
    }

    Ok(())
}
