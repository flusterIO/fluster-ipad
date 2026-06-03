pub mod codegen;
pub mod documentation;
pub mod errors;
pub mod methods;
pub mod traits;
pub mod workspace_utils;

use crate::{
    codegen::parsers::color_parser_template::ColorParserTemplate,
    methods::{
        css::write_rust_emphasis_parser::RustEmphasisParserTemplate, write_sizable_css::SizableCssTemplate,
        write_supported_syntaxes::write_supported_syntaxes,
        write_supported_syntaxes_rust::write_supported_syntaxes_rust,
    },
    traits::DocGenTemplate,
};

#[tokio::main]
async fn main() {
    if let Err(err) = write_supported_syntaxes() {
        eprintln!("Error: {:#?}", err);
    }
    write_supported_syntaxes_rust();
    SizableCssTemplate::generate();
    RustEmphasisParserTemplate::gather_data().generate(
"packages/rust/conundrum/src/lang/lib/ui/ui_types/emphasis/emphasis_parser.rs".to_string()
    )
                                             .expect("Writes rust emphasis parser without throwing an error.");
    ColorParserTemplate::gather_data().generate("packages/rust/conundrum/src/parsers/conundrum/color/color_parser.rs".to_string()).expect("Writes color parser without throwing an error.");
    // RustEmphasisToColorGroupTemplate::gather_data().generate().expect("Writes
    // rust emphasis to css color group without throwing an error.");
}
