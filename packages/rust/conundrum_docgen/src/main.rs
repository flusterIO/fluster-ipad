pub mod methods;
pub mod workspace_utils;
use askama::Template;
use methods::write_supported_syntaxes;

use crate::methods::{
    write_sizable_css::SizableCssTemplate, write_supported_syntaxes::write_supported_syntaxes,
    write_supported_syntaxes_rust::write_supported_syntaxes_rust,
};

#[tokio::main]
async fn main() {
    if let Err(err) = write_supported_syntaxes() {
        eprintln!("Error: {:#?}", err);
    }
    write_supported_syntaxes_rust();
    SizableCssTemplate::generate();
}
