use clap::{Parser, Subcommand};

use crate::{
    commands::{
        compile_directory_from_config::compile_directory, parse_conundrum::parse_conundrum,
        parse_directory_to_directory::get_directory_conundrum_files, watch::watch_directory,
    },
    environments::web::next::write_next_output,
    models::config::CliConfig,
};
mod commands;
mod environments;
mod errors;
mod models;
mod utils;

/// A simple CLI application built with Clap.
#[derive(Parser, Debug)]
#[command(author, version, name="cdrm",  about="The early stages of a Conundrum cli & TUI, because sometimes your thoughts come faster than a full-scale application can move.", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
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
    CompileDirectory {
        config: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match &args.command {
        Some(Commands::ParseConundrum { file_path,
                                        output, }) => {
            let _ = parse_conundrum(file_path.as_str(), output.as_str()).await;
        }
        Some(Commands::CompileDirectory { config, }) => {
            if let Ok(config) = CliConfig::read(config) {
                let err = compile_directory(&config).await;
                if err.is_err() {
                    eprintln!("Error: {:#?}", err.err());
                }
            } else {
                println!("There was an error parsing your config. Conundrum is still in it's very early stages, so this might be an issue on our end and there unfortunately isn't much documentation yet. If you're familiar with Rust, you can examine the `CliConfig` type, as that is exactly the structure of the json file.");
            }
        }
        Some(Commands::WatchDirectory { config, }) => {
            if let Ok(config) = CliConfig::read(config) {
                let err = watch_directory(&config).await;
                if err.is_err() {
                    eprintln!("Error: {:#?}", err.err());
                }
            } else {
                println!("There was an error parsing your config. Conundrum is still in it's very early stages, so this might be an issue on our end and there unfortunately isn't much documentation yet. If you're familiar with Rust, you can examine the `CliConfig` type, as that is exactly the structure of the json file.")
            }
        }
        None => {
            println!("No command provided. Use --help for usage.");
        }
    }
}
