use conundrum::lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum};

#[tokio::test]
async fn parses_code_block() {
    let test_content = r#"
```js -- title="webview_container_view.swift"
// Some Comment: Do some stuff...
```
        "#;

    let res = run_conundrum(ParseConundrumOptions { note_id: None,
        content: test_content.to_string(),
        hide_components: Vec::new(),
        modifiers: Vec::new(),
        ..Default::default()
    }).await.expect("Returns a vald result when a valid input was provided.");

    insta::assert_snapshot!(res.content);
}
