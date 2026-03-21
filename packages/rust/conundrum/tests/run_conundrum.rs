use conundrum::{
    lang::runtime::run_conundrum::{ParseMdxOptions, run_conundrum},
    testing::get_test_content::get_test_content,
};
use fluster_core_utilities::core_types::{
    component_constants::{
        auto_inserted_component_name::AutoInsertedComponentName, component_names::EmbeddableComponentName,
    },
    documentation_constants::in_content_documentation_id::InContentDocumentationId,
};
use strum::IntoEnumIterator;
mod parser_tests;

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

    assert!(res.content
               .contains(&format!("<{}", &AutoInsertedComponentName::FlusterAiParsePendingContainer.to_string())),
            "Conundrum parses Ai Parsing Request components");

    println!("{}", res.content);

    // assert!(res.content.contains("<FlusterCitation"))

    // insta::assert_snapshot!(&res.content);
}

#[tokio::test]
async fn conundrum_parses_documentation_requests() {
    for name in EmbeddableComponentName::iter() {
        let short_test_content = format!("# My markdown\n\n{}?", name);
        let full_test_content = format!("# My markdown\n\n{}??", name);

        let short_opts = ParseMdxOptions { note_id: None,
                                           content: short_test_content,
                                           citations: Vec::new() };

        let short_res = run_conundrum(short_opts).await;

        println!("Short: {}", short_res.content);

        let full_opts = ParseMdxOptions { note_id: None,
                                          content: full_test_content,
                                          citations: Vec::new() };

        let full_res = run_conundrum(full_opts).await;

        assert!(short_res.content
                         .contains(&format!("<InContentDocumentationContainer componentName=\"{}\" format=\"short\">",
                                            name.to_component_id())),
                "Conundrum parses the proper 'short' documentation request for {}.",
                name);

        assert!(full_res.content
                        .contains(&format!("<InContentDocumentationContainer componentName=\"{}\" format=\"full\">",
                                           name.to_component_id())),
                "Conundrum parses the proper 'full' documentation request for {}.",
                name);
    }

    for id in InContentDocumentationId::iter() {
        let short_test_content = format!("# My markdown\n\n{}?", id);
        let full_test_content = format!("# My markdown\n\n{}??", id);

        let short_opts = ParseMdxOptions { note_id: None,
                                           content: short_test_content,
                                           citations: Vec::new() };

        let short_res = run_conundrum(short_opts).await;

        let full_opts = ParseMdxOptions { note_id: None,
                                          content: full_test_content,
                                          citations: Vec::new() };

        let full_res = run_conundrum(full_opts).await;

        assert!(short_res.content
                         .contains(&format!("<InContentDocumentationContainer inContentId=\"{}\" format=\"short\">",
                                            id)),
                "Conundrum parses the proper 'short' documentation request for {}.",
                id);

        assert!(full_res.content
                        .contains(&format!("<InContentDocumentationContainer inContentId=\"{}\" format=\"full\">",
                                           id)),
                "Conundrum parses the proper 'full' documentation request for {}.",
                id);
    }
}
