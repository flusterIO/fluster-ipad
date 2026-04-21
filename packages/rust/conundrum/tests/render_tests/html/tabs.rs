use crate::render_tests::render_test::render_test;

#[tokio::test]
async fn renders_tabgroup_to_html() {
    let test_content = r#"<Tabs>
<Tab label="My tab one">
My tab here
</Tab>
<Tab label="My tab two">
My tab two
</Tab>
<Tab label="My tab three">
My tab three
</Tab>
</Tabs>"#;
    render_test(test_content, "python-code-block").await;
}
