use std::thread;

use crate::commands::parse_conundrum::parse_conundrum;
use crate::errors::ConundrumCliError;

pub async fn parse_directory_to_directory(input_dir: &str, output_dir: &str) -> Result<(), std::io::Error> {
    let files = std::fs::read_dir(input_dir)?;
    for f in files {
        let x = f.map_err(|_| ConundrumCliError::FsError)?;
        // let input_path =
        // let handle  = thread::spawn(move || {
        //        // parse_conundrum()
        // });
    }
}
