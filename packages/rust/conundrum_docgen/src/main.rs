#![feature(trim_prefix_suffix)]
pub mod codegen;
pub mod documentation;
pub mod errors;
mod generated_generators;
mod generator_context;
pub mod methods;
pub mod traits;
pub mod workspace_utils;

use crate::{
    codegen::{
        parsers::color_parser_template::ColorParserTemplate,
        templates::{
            emphasis_variable_match::EmphasisVariableMatch, initial_note_paths_swift::InitialNotePathsSwift,
            mcp_tool_names::MCPToolNameList,
        },
    },
    documentation::{emphasis::EmphasisDocs, highlight::HighlightDocs, underline::UnderlineDocs},
    errors::DocGenError,
    generated_generators::docgen_generators::run_generated_generators,
    methods::{
        css::write_rust_emphasis_parser::RustEmphasisParserTemplate, json_docs::parse_json_docs::parse_json_docs,
        write_sizable_css::SizableCssTemplate, write_supported_syntaxes::write_supported_syntaxes,
        write_supported_syntaxes_rust::write_supported_syntaxes_rust,
    },
    traits::DocGenTemplate,
    workspace_utils::get_workspace_root_duplicate::get_workspace_root,
};
use conundrum::ecosystem::environment_variables::cdrm_env_variable::CdrmEnvVariable;
use rustdoc_json::Builder;

#[tokio::main]
async fn main() {
    let env_level = CdrmEnvVariable::LogLevel.read().map(|x| x.to_lowercase()).unwrap_or("warn".to_string());
    let filters = format!("warn,conundrum_server_rs={},conundrum={},conundrum_db={},conundrum_fs={}",
                          &env_level, &env_level, &env_level, &env_level);
    pretty_env_logger::formatted_builder().parse_filters(filters.as_str()).init();
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
    UnderlineDocs::gather_data().generate("docs/in_content_docs/components/underline.mdx".to_string())
                                .expect("Writes underline docs without throwing an error.");
    HighlightDocs::gather_data().generate("docs/in_content_docs/components/highlight.mdx".to_string())
                                .expect("Writes highlight docs without throwing an error.");
    MCPToolNameList::gather_data().generate("packages/rust/conundrum_ts/src/code_gen/docgen/mcp_tool_names.ts".to_string())
        .expect("Failed to generated the MCPToolNameList typescript output.");
    InitialNotePathsSwift::gather_data().generate("packages/swift/FlusterData/Sources/FlusterData/constants/initial_note_paths.swift".to_string())
                                        .expect("Writes initial note paths to Swift");
    let root = get_workspace_root();
    let cdrm_path = std::path::Path::new(&root).join("packages").join("rust").join("conundrum").join("Cargo.toml");
    let output_path = Builder::default().manifest_path(cdrm_path).build().unwrap();
    println!("Wrote conundrum docs as json to {:?}", output_path);
    run_generated_generators().map_err(|e| {
                                  log::error!("Error: {}", e);
                                  DocGenError::GeneralError
                              })
                              .expect("Failed to run generated generators");
    log::info!("Wrote generated generator content.");
    // parse_json_docs(output_path.to_str().unwrap()).inspect_err(|e| {
    //                                                   eprintln!("Error:
    // {:#?}", e);                                               });
}
