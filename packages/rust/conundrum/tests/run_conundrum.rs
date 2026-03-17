use conundrum::{
    lang::runtime::run_conundrum::{ParseMdxOptions, run_conundrum},
    testing::get_test_content::get_test_content,
};
use fluster_core_utilities::core_types::component_constants::{
    auto_inserted_component_name::AutoInsertedComponentName, documentation_component_name::DocumentationComponentName,
};

#[tokio::test]
async fn runs_conundrum() {
    let test_content = get_test_content("full_test.mdx");

    let opts = ParseMdxOptions { note_id: None,
                                 content: test_content,
                                 citations: Vec::new() };

    let res = run_conundrum(opts).await;

    print!("{}", res.content);
    assert!(!&res.content.is_empty(), "Conundrum response is not empty.");

    assert!(res.content.contains("<DictionaryEntry"), "Conundrum parses DictionaryEntry components");
    assert!(res.content.contains("<NoteLink"), "Conundrum parses NoteLink components");
    assert!(res.content.contains("<FlusterCitation"), "Conundrum parses FlusterCitation components");
    assert!(res.content.contains("<AutoInsertedTag"), "Conundrum parses AutoInsertedTag components");
    assert!(res.content.contains("<InContentDocumentationContainer inContentId=\"Docs\" format=\"short\">"),
            "Conundrum parses the proper 'short' documentation request.");
    assert!(res.content.contains("<InContentDocumentationContainer inContentId=\"Docs\" format=\"full\">"),
            "Conundrum parses the proper 'full' documentation request.");

    assert!(res.content
               .contains(&format!("<{}", &AutoInsertedComponentName::FlusterAiParsePendingContainer.to_string())),
            "Conundrum parses Ai Parsing Request components");
    assert!(res.content
               .contains(&format!("<{}", &DocumentationComponentName::InContentDocumentationContainer.to_string())),
            "Conundrum parses Docs components");

    // assert!(res.content.contains("<FlusterCitation"))

    // insta::assert_snapshot!(&res.content);
}
