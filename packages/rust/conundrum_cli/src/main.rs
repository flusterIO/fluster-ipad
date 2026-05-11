use clap::{Parser, Subcommand};

use crate::commands::parse_conundrum::parse_conundrum;
mod commands;
mod utils;
mod errors;

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
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match &args.command {
        Some(Commands::ParseConundrum { file_path,
                                        output, }) => {
            let _ = parse_conundrum(file_path.as_str(), output.as_str()).await;
        }
        None => {
            println!("No command provided. Use --help for usage.");
        }
    }
}
