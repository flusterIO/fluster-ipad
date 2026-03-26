use conundrum::{
    lang::runtime::run_conundrum::{ParseMdxOptions, run_conundrum},
    testing::get_test_content::get_test_content,
};

#[tokio::test]
pub async fn parses_title_with_heading() {
    let test_content = get_test_content("title_with_id.mdx");

    let res = run_conundrum(ParseMdxOptions { note_id: None,
                                              content: test_content.to_string(),
                                              modifiers: Vec::new() }).await;

    insta::assert_snapshot!(res.content);
}
