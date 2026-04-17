use crate::render_tests::render_test::render_test;

#[tokio::test]
async fn renders_codeblock_to_html() {
    let test_content = r#"```python -- title="my_code_block.py"
def my_func():
       return np.linspace(0, smp.pi, 100)
```"#;
    render_test(test_content, "python-code-block").await;
}
