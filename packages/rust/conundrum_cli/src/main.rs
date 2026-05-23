use crate::commands::{compile_project::compile_directory, parse_conundrum::parse_conundrum, watch::watch_directory};
use clap::{Parser, Subcommand};
use conundrum_config::{ecosystem::project::project_config::ProjectConfig, traits::config_file::ConfigFile};
mod commands;
mod environments;
mod errors;
mod models;
mod utils;
use clap_verbosity::{InfoLevel, Verbosity};

/// A simple CLI application built with Clap.
#[derive(Parser, Debug)]
#[command(author, version, name="cdrm",  about="The early stages of a Conundrum cli & TUI, because sometimes your thoughts come faster than a full-scale application can move.", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    ParseConundrum {
        file_path: String,
        output: String,
    },
    WatchDirectory {
        config: Option<String>,
    },
    CompileProject {
        config: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    match &args.command {
        Some(Commands::ParseConundrum { file_path,
                                        output, }) => {
            let _ = parse_conundrum(file_path.as_str(), output.as_str()).await;
        }
        Some(Commands::CompileProject { config, }) => match ProjectConfig::read(config) {
            Ok(config) => {
                let err = compile_directory(&config).await;
                if err.is_err() {
                    eprintln!("Error: {:#?}", err.err());
                }
            }
            Err(err) => {
                println!("There was an error parsing your config. Conundrum is still in it's very early stages, so this might be an issue on our end and there unfortunately isn't much documentation yet. If you're familiar with Rust, you can examine the `ProjectConfig` type, as that is exactly the structure of the json file.\n\nError: {:#?}",
                         err);
            }
        },
        Some(Commands::WatchDirectory { config, }) => {
            if let Ok(config) = ProjectConfig::read(&config) {
                let err = watch_directory(&config).await;
                if err.is_err() {
                    eprintln!("Error: {:#?}", err.err());
                }
            } else {
                println!("There was an error parsing your config. Conundrum is still in it's very early stages, so this might be an issue on our end and there unfortunately isn't much documentation yet. If you're familiar with Rust, you can examine the `ProjectConfig` type, as that is exactly the structure of the json file.")
            }
        }
        None => {
            println!("No command provided. Use --help for usage.");
        }
    }
}
