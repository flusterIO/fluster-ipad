use conundrum::{
    lang::runtime::{
        run_conundrum::{ParseConundrumOptions, run_conundrum},
        state::parse_state::ConundrumCompileTarget,
    },
    testing::test_result::TestResult,
};

#[tokio::test]
async fn parses_simple_table() {
    let test_content = r#"| Header 1 (Left) | Header 2 (Center) | Header 3 (Right) | Header 4 (Justify) | Header 5 (Default) |
|:-----------------|:-----------------:|-----------------:|---------------------|---------------------|
| My table cell one | My cell two | 3.1415 | My table cell here | Here's another cell |
| My table cell one | My cell two | 3.1415 | My table cell here | Here's another cell |
| My table cell one | My cell two | 3.1415 | My table cell here | Here's another cell |"#;

    let res = run_conundrum(ParseConundrumOptions { note_id: None,
        content: test_content.to_string(),
        hide_components: Vec::new(),
        modifiers: Vec::new(),
        target: ConundrumCompileTarget::Html,
        ..Default::default()
    }).inspect_err(|e| {
        println!("Error: {:#?}", e);
    }).expect("Returns a vald result when a valid input was provided.");

    println!("Res: {:#?}", res);
    let t = TestResult(res);

    let table_count = t.count_css_query("table");
    assert!(table_count == 1, "Finds one table element in the rendered html");
}

#[tokio::test]
async fn parses_semi_complex_table_with_variety_of_syntaxes() {
    let test_content = r#"
| Header 1 (Left) | Header 2 (Center) | Header 3 (Right) | Header 4 (Justify) | Header 5 (Default) |
|:-----------------|:-----------------:|-----------------:|---------------------|---------------------|
| **Bold** & *Italic* with [Link](https://example.com) and ![Image](https://example.com/image.jpg) | Line break<br>with two spaces: line1\n line2 | `Code snippet` and ```python\nprint("Hello")\n``` | \* \| \# \& and *nested **bold** with [link](#)* | *Italic* **Bold** [Link](url) ![Image](url) @ # $ |
| A long text with multiple line breaks and emojis 😊 | - List item 1<br>- List item 2 | ```python\nprint("Hello")\n``` and `inline code` | $E = mc^2$ | - Level 1<br>  - Level 2<br>    - Level 3 |
| > Blockquote: This is a blockquote. | --- | ~~Strikethrough~~ | [ ] Task 1<br>[x] Task 2 | --- title: Test --- |
| Combination of all: **Bold**, *Italic*, [Link](url), ![Image](url), `code`, line break, 😊, and `escaped \* \| \# \&` | |---|---| | - *Italic* and **Bold** | $E = mc^2$ | Mix of all elements again |
"#;

    let res = run_conundrum(ParseConundrumOptions { note_id: None,
        content: test_content.to_string(),
        hide_components: Vec::new(),
        modifiers: Vec::new(),
        ..Default::default()
    }).inspect_err(|e| {
        println!("Error: {:#?}", e);
    }).expect("Returns a vald result when a valid input was provided.");

    let t = TestResult(res);

    let table_count = t.count_css_query("table");

    assert!(table_count == 1, "Finds one table element in the rendered html");
}
