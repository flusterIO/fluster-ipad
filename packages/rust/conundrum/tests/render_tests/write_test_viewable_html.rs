use std::sync::Arc;

use askama::Template;
use conundrum::{
    lang::runtime::{
        parse_conundrum_string::parse_elements,
        run_conundrum::{ParseConundrumOptions, run_conundrum},
        state::{
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            parse_state::{ConundrumModifier, ParseState},
        },
        traits::conundrum_input::ConundrumInput,
    },
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};
use fluster_core_utilities::test_utilities::get_workspace_root::get_workspace_root;
use parking_lot::RwLock;
use winnow::error::ErrMode;

use crate::runs_conundrum;

#[derive(Template)]
#[template(ext = "jinja", path = "test_sample_page.jinja")]
struct TestPageTemplate {
    pub content: String,
    pub label: String,
}

pub async fn write_test_html(content: &str, label: &str) -> ConundrumModalResult<()> {
    let res =
        run_conundrum(ParseConundrumOptions { content: content.to_string(),
                                              ..Default::default() }).await
                                                                     .expect("Parses input without throwing an error.");

    let templ = TestPageTemplate { content: res.content.to_string(),
                                   label: label.to_string() };
    let content = templ.render().expect("Test template renders without throwing an error.");

    let repo_root = get_workspace_root();
    let output_path = std::path::Path::new(&repo_root).join("packages")
                                                      .join("rust")
                                                      .join("conundrum")
                                                      .join("test_output_viewable.html");
    std::fs::write(output_path, content);
    Ok(())
}
