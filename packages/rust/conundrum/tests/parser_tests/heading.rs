use std::ops::Index;

use conundrum::{
    lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum},
    testing::get_test_content::get_test_content,
};

#[tokio::test]
pub async fn parses_title_with_heading() {
    let test_content = get_test_content("title_with_id.mdx");

    let res = run_conundrum(ParseConundrumOptions { note_id: None,
        content: test_content.to_string(),
        hide_components: Vec::new(),
        modifiers: Vec::new(),
        ..Default::default()
    }).expect("Returns a vald result when a valid input was provided.");

    assert!(!res.toc.is_empty(), "Appends the heading to the table of contents.");
    assert!(res.toc.index(0).id == "myIdHere", "Sets the proper id in the table of contents.");
    assert!(res.toc.index(0).content.as_str() == "Title with id", "Finds the proper title content.");

    insta::assert_snapshot!(res.content);
}
