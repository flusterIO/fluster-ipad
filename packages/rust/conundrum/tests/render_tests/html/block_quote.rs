use conundrum::testing::render_test::render_test;

#[tokio::test]
async fn renders_block_quote_to_html() {
    let test_content = r#"## My content

> My block quote
> With a nested line
>> And a nested block quote.  
>> With another line.
"#;
    render_test(test_content, "Admonition").await;
}
