#![feature(trim_prefix_suffix)]
pub mod codegen;
pub mod documentation;
pub mod errors;
pub mod methods;
pub mod traits;
pub mod workspace_utils;

use crate::{
    codegen::{
        parsers::color_parser_template::ColorParserTemplate,
        templates::{emphasis_variable_match::EmphasisVariableMatch, initial_note_paths_swift::InitialNotePathsSwift},
    },
    documentation::emphasis::EmphasisDocs,
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
    EmphasisVariableMatch::gather_data().generate("packages/rust/conundrum/src/lang/lib/ui/ui_types/emphasis/variable_to_emphasis.rs".to_string()).expect("Writes css variable to emphasis without throwing an error.");
    EmphasisDocs::gather_data().generate("docs/in_content_docs/emphasis-docs.mdx".to_string())
                               .expect("Writes emphasis docs without throwing an error.");
    InitialNotePathsSwift::gather_data().generate("packages/swift/FlusterData/Sources/FlusterData/constants/initial_note_paths.swift".to_string())
                                        .expect("Writes initial note paths to Swift");
    // RustEmphasisToColorGroupTemplate::gather_data().generate().expect("Writes
    // rust emphasis to css color group without throwing an error.");
}
