use conundrum::testing::{render_test::render_test, write_test_ast::write_test_ast};

#[tokio::test]
async fn renders_codeblock_to_html() {
    let test_content = r#"# My title 1 <Emoji smedium inline name="rocket"/>

- My item here
  - My nested item
- My other item.
  - My other item here
  - My item here
    - My other nested item

"#;
    write_test_ast(test_content, "renders-basic-lists").expect("Renders plain list without throwing an error.");
    render_test(test_content, "renders-basic-lists").await;
}
