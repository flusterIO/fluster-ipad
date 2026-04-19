use crate::render_tests::render_test::render_test;

#[tokio::test]
async fn renders_codeblock_to_html() {
    let test_content = r#"# My title
> My subtitle

My paragraph goes here.

$$
\delta = 2 G \frac{M_\oplus}{R_\oplus}\hat{R}
$$

```python -- title="my_code_block.py"
def my_func():
       return np.linspace(0, smp.pi, 100)
```"#;
    render_test(test_content, "python-code-block").await;
}
