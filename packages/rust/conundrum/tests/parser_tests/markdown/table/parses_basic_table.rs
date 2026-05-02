use conundrum::{
    lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum},
    parsers::conundrum::logic::number::conundrum_int::ConundrumInt,
    testing::render_test::render_test,
};

#[tokio::test]
async fn parses_simple_table_with_variety_of_syntaxes() {
    let test_content = r#"
| Header 1 (Left) | Header 2 (Center) | Header 3 (Right) | Header 4 (Justify) | Header 5 (Default) |
|:-----------------|:-----------------:|-----------------:|---------------------|---------------------|
| **Bold** & *Italic* with [Link](https://example.com) and ![Image](https://example.com/image.jpg) | Line break<br>with two spaces: line1\n line2 | `Code snippet` and ```python\nprint("Hello")\n``` | \* \| \# \& and *nested **bold** with [link](#)* | *Italic* **Bold** [Link](url) ![Image](url) @ # $ |
| A long text with multiple line breaks and emojis 😊 | - List item 1<br>- List item 2 | ```python\nprint("Hello")\n``` and `inline code` | $E = mc^2$ | - Level 1<br>  - Level 2<br>    - Level 3 |
| > Blockquote: This is a blockquote. | --- | ~~Strikethrough~~ | [ ] Task 1<br>[x] Task 2 | --- title: Test --- |
| Combination of all: **Bold**, *Italic*, [Link](url), ![Image](url), `code`, line break, 😊, and `escaped \* \| \# \&` | |---|---| | - *Italic* and **Bold** | $E = mc^2$ | Mix of all elements again |
| Very long text with @ # $ % ^ & * ( ) | ```python\nprint("Hello")\n``` | - *Italic* and **Bold** | |---|---| | Mix of all elements again |iiaia
"#;

    let res = run_conundrum(ParseConundrumOptions { note_id: None,
        content: test_content.to_string(),
        hide_components: Vec::new(),
        modifiers: Vec::new(),
        ..Default::default()
    }).inspect_err(|e| {
        println!("Error: {:#?}", e);
    }).expect("Returns a vald result when a valid input was provided.");

    println!("Res: {:#?}", res);
    assert!(res.footnotes.contains_key(&ConundrumInt(1)), "Found the footnote as expected.");
}
