use crate::{
    lang::runtime::{
        run_conundrum::{ParseConundrumOptions, run_conundrum},
        state::conundrum_error_variant::ConundrumModalResult,
    },
    testing::get_workspace_root::get_workspace_root,
};
use askama::Template;

#[derive(Template)]
#[template(ext = "jinja", path = "test_sample_page.jinja")]
struct TestPageTemplate {
    pub content: String,
    pub label: String,
}

pub async fn write_test_html(content: &str, label: &str) -> ConundrumModalResult<()> {
    let res =
        run_conundrum(ParseConundrumOptions { content: content.to_string(),
                                              ..Default::default() }).expect("Parses input without throwing an error.");

    let templ = TestPageTemplate { content: res.content.to_string(),
                                   label: label.to_string() };
    let content = templ.render().expect("Test template renders without throwing an error.");

    let repo_root = get_workspace_root();
    let output_path = std::path::Path::new(&repo_root).join("packages")
                                                      .join("rust")
                                                      .join("conundrum")
                                                      .join("test_output_viewable.html");
    std::fs::write(output_path, content).expect("Writes file without throwing an error.");
    Ok(())
}
