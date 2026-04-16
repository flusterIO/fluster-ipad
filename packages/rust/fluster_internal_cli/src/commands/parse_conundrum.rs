use std::fs;

use conundrum::lang::runtime::{
    run_conundrum::{ParseConundrumOptions, run_conundrum},
    state::ui_params::UIParams,
};

/// Definitely moving this to the Conundrum cli, but need this for generating
/// documentation for now.
pub async fn parse_conundrum(file_path: &str, output: &str) -> std::io::Result<()> {
    println!("File Path: {file_path}");
    let content = fs::read_to_string(file_path)?;
    let p = run_conundrum(ParseConundrumOptions { content,
                                            note_id: None,
                                            modifiers: Vec::new(),
                                            ui_params: UIParams::default(),
                                            hide_components: Vec::new()
    }).await.expect("Returns a vald result when a valid input was provided.");
    println!("Content: {}", p.content);
    fs::write(output, p.content)?;
    Ok(())
}
