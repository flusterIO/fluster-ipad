use crate::render_tests::{
    render_test::render_test, write_test_ast::write_test_ast, write_test_viewable_html::write_test_html,
};

#[tokio::test]
async fn renders_codeblock_to_html() {
    let test_content = r#"# My title <Emoji smedium inline name="rocket"/>
> My subtitle

My paragraph goes here with inline $e=mc^2$ math.
My paragraph without a new <Emoji smedium name="smile" inline /> line that makes me 

My paragraph _after_ **two** ***new lines***.  
My paragraph :smile: after two spaces and a line break.

"#;
    write_test_ast(test_content, "semi-complete").expect("Writes ast successfully.");
    write_test_html(test_content, "semi-complete codeblock").await.expect("Writes html successfully.");
    render_test(test_content, "python-code-block").await;
}
