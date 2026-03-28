use std::ops::Index;

use conundrum::{
    lang::runtime::run_conundrum::{ParseMdxOptions, run_conundrum},
    testing::get_test_content::get_test_content,
};
use rssn::prelude::argmin::seq::IndexedRandom;

#[tokio::test]
pub async fn parses_title_with_heading() {
    let test_content = get_test_content("title_with_id.mdx");

    let res = run_conundrum(ParseMdxOptions { note_id: None,
                                              content: test_content.to_string(),
                                              modifiers: Vec::new() }).await;

    assert!(!res.toc.is_empty(), "Appends the heading to the table of contents.");
    assert!(res.toc.index(0).id.as_ref().is_some_and(|s| s == "myIdHere"),
            "Sets the proper id in the table of contents.");
    assert!(res.toc.index(0).content.as_str() == "Title with id", "Finds the proper title content.");

    insta::assert_snapshot!(res.content);
}
