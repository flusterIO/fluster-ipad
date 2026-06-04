use conundrum::testing::render_test::render_test;

use crate::render_tests::documentation::get_documentation_content::get_documentation_content;

#[tokio::test]
async fn renders_emphasis_docs_to_html() {
    let test_content =
        &get_documentation_content("packages/rust/conundrum/src/embedded/in_content_docs/emphasis-docs-full.mdx");
    render_test(test_content, "Emphasis Docs").await;
}
