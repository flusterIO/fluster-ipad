use conundrum::{
    lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum},
    testing::render_sample,
};

#[tokio::test]
pub async fn allows_escape_math() {
    let test_content = r#"
1. Parse all content _linearly_. Don't worry about multi-threading during the parsing phase.
    - All state must be set during the parsing phase.
2. The compilation phase can then be multi-threaded, using a Rayon iterator to take \$advantage$ of the entire system.

--- <Emoji name="heart" smedium /> ---

$$
e=mc^2
$$

<Admonition title="My admonition" foldable>

$$
e=mc^2
$$

</Admonition>

$$
e=mc^2
$$


$$
e=mc^2
$$


<Tabs> 
    <Tab label="Tab 1">
        $$
      e=mc^2
      $$
    </Tab>
    <Tab label="Tab Label 2">
        $$
      e=mc^2
      $$
    </Tab>
    <Tab label="Tab Label 3">
        $$
      e=mc^2
      $$
    </Tab>
</Tabs>


      $$
      e=mc^2
      $$
        "#;
    let res = run_conundrum(
           ParseConundrumOptions {
               content: test_content.to_string(),
               note_id: None,
               modifiers: Vec::new(),
               hide_components: Vec::new(),
               ..Default::default()
           }
       ).await.expect("Runs conundrum without throwing an error.");
    insta::assert_snapshot!(res.content);
    render_sample::TestRenderPage::new_run(test_content, "Nested Math State");
}
