use std::sync::Arc;

use conundrum::{
    lang::runtime::{
        parse_conundrum_string::parse_elements,
        state::{
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            parse_state::ParseState,
        },
        traits::conundrum_input::ConundrumInput,
    },
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};
use fluster_core_utilities::test_utilities::get_workspace_root::get_workspace_root;
use parking_lot::RwLock;
use winnow::error::ErrMode;

pub fn write_test_ast(content: &str, label: &str) -> ConundrumModalResult<()> {
    let mut input =
        ConundrumInput { input: content,
                         state:
                             Arc::new(RwLock::new(ParseState { data:
                                                                   MdxParsingResult::from_initial_mdx_content(content),
                                                               ..Default::default() })) };
    let tree = parse_elements(&mut input).map_err(|e| {
                                             panic!("Parses {} content without throwing an error.", label);
                                         })
                                         .map_err(|e| ErrMode::Backtrack(ConundrumErrorVariant::FailToGenerateString))?;
    let json_string = serde_json::to_string_pretty(&tree).expect("Serializes AST to JSON successfully.");
    let repo_root = get_workspace_root();
    let output_path =
        std::path::Path::new(&repo_root).join("packages").join("rust").join("conundrum").join("test_output_ast.json");
    std::fs::write(output_path, json_string);
    Ok(())
}
