use conundrum::testing::render_test::render_test;

#[tokio::test]
async fn renders_admonitions_to_html() {
    let test_content = r#"# My title

<Color color="#f00" />

<Color error />

    "#;
    render_test(test_content, "Admonition").await;
}
