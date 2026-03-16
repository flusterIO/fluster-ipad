use conundrum::{
    lang::runtime::run_conundrum::{ParseMdxOptions, run_conundrum},
    testing::get_test_content::get_test_content,
};

#[tokio::test]
async fn runs_conundrum() {
    let test_content = get_test_content("full_test.mdx");

    let opts = ParseMdxOptions {
        note_id: None,
        content: test_content,
        citations: Vec::new(),
    };

    let res = run_conundrum(opts).await;

    println!("Response: {:#?}", res);
    insta::assert_snapshot!(&res.content);
    // assert_eq!(result, 4);
}
