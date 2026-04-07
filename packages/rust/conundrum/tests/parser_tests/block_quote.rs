use conundrum::lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum};

#[tokio::test]
async fn parses_block_quote_of_depth_1() {
    let test_content = r#"
> My block quote here
> and here, and here,
> and also here.
        "#;

    let res = run_conundrum(ParseConundrumOptions { note_id: None,
                                              content: test_content.to_string(),
                                              modifiers: Vec::new() }).await.expect("Returns a vald result when a valid input was provided.");

    insta::assert_snapshot!(res.content);
}

#[tokio::test]
async fn parses_block_quote_of_nested_depth() {
    let test_content = r#"
> My block quote here
> and here, and here, $e=mc^2$
>> and also here.
>> and here.
> And this is back in the main block quote.
        "#;

    let res = run_conundrum(ParseConundrumOptions { note_id: None,
                                              content: test_content.to_string(),
                                              modifiers: Vec::new() }).await.expect("Returns a vald result when a valid input was provided.");

    println!("{}", res.content)
}
