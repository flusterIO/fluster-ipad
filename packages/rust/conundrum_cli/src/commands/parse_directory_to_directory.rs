use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use conundrum_config::ecosystem::project::project_config::ProjectConfig;
use ignore::WalkBuilder;

use crate::{
    errors::{ConundrumCliError, ConundrumCliResult},
    models::conundrum_file::ConundrumFile,
};

/// Source files with these extensions are parsed; everything else is ignored.
const CONUNDRUM_EXTENSIONS: &[&str] = &["cdrm", "mdx", "md"];

/// Lives in the output directory. One `<content hash>\t<relative source path>` line per file that was
/// successfully parsed on the previous run.
const MANIFEST_FILE_NAME: &str = ".cdrm-manifest";

/// FNV-1a. Unlike `DefaultHasher` this is stable across runs and Rust versions, which matters because the
/// hashes are persisted between invocations.
fn hash_content(content: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in content.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn read_manifest(path: &Path) -> HashMap<String, u64> {
    let Ok(raw) = fs::read_to_string(path) else {
        return HashMap::new();
    };
    raw.lines()
       .filter_map(|line| {
           let (hash, rel) = line.split_once('\t')?;
           Some((rel.to_string(), u64::from_str_radix(hash, 16).ok()?))
       })
       .collect()
}

fn write_manifest(path: &Path, entries: &HashMap<String, u64>) -> ConundrumCliResult<()> {
    let mut lines: Vec<String> = entries.iter().map(|(rel, hash)| format!("{hash:016x}\t{rel}")).collect();
    lines.sort();
    fs::write(path, lines.join("\n")).map_err(|e| {
        eprintln!("Error: {:#?}", e);
        ConundrumCliError::FsError(path.to_str().map(String::from).unwrap_or_default())
    })
}

fn is_conundrum_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| CONUNDRUM_EXTENSIONS.iter().any(|x| x.eq_ignore_ascii_case(e)))
}

/// Parses the Conundrum files in `input_dir` and writes the results to `output_dir`, mirroring the directory
/// structure.
///
/// Parsing is incremental: a content hash of every source file is stored in `output_dir/.cdrm-manifest`, and a
/// file is only re-parsed when its hash differs from the last run or its output file no longer exists.
/// Only the files that were actually re-parsed are returned.
pub async fn parse_directory_to_directory(input_dir: &str,
                                          output_dir: &str,
                                          cfg: &ProjectConfig)
                                          -> ConundrumCliResult<Vec<ConundrumFile>> {
    let input_path = Path::new(input_dir);
    if !input_path.is_dir() {
        return Err(ConundrumCliError::NotADirectory(input_dir.to_string()));
    }
    fs::create_dir_all(output_dir).map_err(|_| ConundrumCliError::FsError(output_dir.to_string()))?;

    let manifest_path = Path::new(output_dir).join(MANIFEST_FILE_NAME);
    let previous = read_manifest(&manifest_path);
    // Rebuilt from scratch so entries for deleted or failing sources fall out.
    let mut current: HashMap<String, u64> = HashMap::new();

    let mut cdrm_files: Vec<ConundrumFile> = Vec::new();
    let mut first_error: Option<ConundrumCliError> = None;

    for entry in WalkBuilder::new(input_path).build() {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error: {:#?}", e);
                continue;
            }
        };
        let path: PathBuf = entry.path().to_path_buf();
        if !path.is_file() || !is_conundrum_file(&path) {
            continue;
        }
        let path_str = path.to_str().map(String::from).unwrap_or_default();
        let Ok(rel) = path.strip_prefix(input_path) else { continue };
        let rel_key = rel.to_string_lossy().into_owned();

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => {
                first_error.get_or_insert(ConundrumCliError::FsError(path_str));
                continue;
            }
        };
        let hash = hash_content(&content);

        let mut output_path = Path::new(output_dir).join(rel);
        output_path.set_extension(cfg.opts.target.to_file_ext());

        if previous.get(&rel_key) == Some(&hash) && output_path.is_file() {
            current.insert(rel_key, hash);
            continue;
        }

        let result = ConundrumFile::from_absolute_path(&path_str, cfg).and_then(|file| {
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent).map_err(|_| ConundrumCliError::FsError(parent.to_string_lossy().into_owned()))?;
            }
            file.write_to_relative_directory(input_dir, output_dir, cfg)?;
            Ok(file)
        });
        match result {
            Ok(file) => {
                current.insert(rel_key, hash);
                cdrm_files.push(file);
            }
            Err(e) => {
                eprintln!("Failed to parse {path_str}: {e}");
                first_error.get_or_insert(e);
            }
        }
    }

    write_manifest(&manifest_path, &current)?;

    match first_error {
        Some(e) => Err(e),
        None => Ok(cdrm_files),
    }
}
