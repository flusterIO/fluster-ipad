use crate::render_tests::{
    render_test::render_test, write_test_ast::write_test_ast, write_test_viewable_html::write_test_html,
};

#[tokio::test]
async fn renders_block_quote_to_html() {
    let test_content = r#"## My content

> My block quote
> With a nested line
>> And a nested block quote.  
>> With another line.
"#;
    write_test_ast(test_content, "semi-complete").expect("Writes ast successfully.");
    write_test_html(test_content, "semi-complete codeblock").await;
    render_test(test_content, "Admonition").await;
}
