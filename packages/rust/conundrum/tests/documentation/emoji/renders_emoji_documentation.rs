use conundrum::{
    lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum},
    testing::wrap_test_content::wrap_test_conundrum_content,
};

#[tokio::test]
pub async fn renders_emoji_documentation() {
    let test_input = wrap_test_conundrum_content("Emoji??");

    let res = run_conundrum(ParseConundrumOptions { content: "Emoji??".to_string(),
                                                    modifiers: Vec::new(),
                                                    note_id: None,
                                                    hide_components: Vec::new(),
                                                    ..Default::default()
    }).await.expect("Renders documentation without throwing an error.");

    insta::assert_snapshot!(res.content);
}
