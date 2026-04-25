use conundrum::lang::runtime::{
    run_conundrum::{ParseConundrumOptions, run_conundrum},
    state::parse_state::ConundrumModifier,
};

#[tokio::test]
async fn removes_components_for_inline_markdown() {
    let test_content = r#"
# My title <Hl>here</Hl>
        "#;

    let res = run_conundrum(ParseConundrumOptions { note_id: None,
        content: test_content.to_string(),
        hide_components: Vec::new(),
        modifiers: vec![ConundrumModifier::PreferInlineMarkdownSyntax],
        ..Default::default()
    }
    )
        .expect("Returns a vald result when a valid input was provided.");

    insta::assert_snapshot!(res.content);
}
