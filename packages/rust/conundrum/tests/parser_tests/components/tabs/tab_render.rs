use conundrum::lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum};

#[tokio::test]
pub async fn renders_tabs() {
    let test_content = r#"
<Tabs>
  <Tab label="One">
    My tabs content one
  </Tab>
  <Tab label="Two">
    My label content two
  </Tab>
  <Tab label="Three">
   $$
    e=mc^2
   $$
  </Tab>
</Tabs>
"#;

    // let mut test_input = get_conundrum_input(test_content, vec![]);

    let res =
        run_conundrum(ParseConundrumOptions { content: test_content.to_string(),
                                        note_id: None,
                                        hide_components: Vec::new(),
                                        modifiers: Vec::new() }).await
                                                                .expect("Parses tabs input without throwing an error.");

    insta::assert_debug_snapshot!(res.content);
}
