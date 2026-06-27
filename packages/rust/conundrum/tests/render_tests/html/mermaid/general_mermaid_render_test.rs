use conundrum::testing::render_test::render_test;

#[tokio::test]
async fn renders_mermaid_to_html() {
    let test_content = r#"# My title

```mermaid
graph TD
    A[Start] --> B{Do User Action?}
    B -- Yes --> C[Process]
    B -- No --> D[Stop]
```

```mermaid
sequenceDiagram
    participant Alice
    participant Bob

    Alice->>Bob: Hello Bob
    Bob-->>Alice: Hi Alice
```
    "#;
    render_test(test_content, "Mermaid").await;
}
