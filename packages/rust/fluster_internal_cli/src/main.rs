use clap::{Parser, Subcommand};
mod commands;
mod utils;

/// A simple CLI application built with Clap.
#[derive(Parser, Debug)]
#[command(author, version, name="fluster",  about="Internal tools for Fluster development", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Gathers all .fluster_component.json files in the monorepo and saves them in json form to
    /// $FLUSTER_IOS_ROOT/docs/component_docs/.component_doc_paths.json
    GatherComponentDocPaths {},
    ParseInitialNotes {},
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::GatherComponentDocPaths {}) => commands::gather_component_doc_paths::run(),
        Some(Commands::ParseInitialNotes {}) => {
            commands::parse_initial_notes::parse_initial_notes().await;
        }
        // Some(Commands::Calculate { num1, num2 }) => {
        //     println!("{} + {} = {}", num1, num2, num1 + num2);
        // }
        None => {
            println!("No command provided. Use --help for usage.");
        }
    }
}
