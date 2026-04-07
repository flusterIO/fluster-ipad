use conundrum::{
    embedded::in_content_documentation_id::InContentDocumentationId,
    lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum},
    output::general::component_constants::{
        auto_inserted_component_name::AutoInsertedComponentName, component_names::EmbeddableComponentName,
    },
    testing::get_test_content::get_test_content,
};
use strum::IntoEnumIterator;
mod modifier_tests;
mod parser_tests;

#[tokio::test]
async fn runs_conundrum() {
    let test_content = get_test_content("full_test.mdx");

    let opts = ParseConundrumOptions { note_id: None,
                                       content: test_content,
                                       modifiers: Vec::new(),
                                       hide_components: Vec::new() };

    let res = run_conundrum(opts).await.expect("Returns a valid result when vald input was provided.");

    print!("{}", res.content);
    assert!(!&res.content.is_empty(), "Conundrum response is not empty.");

    assert!(!res.dictionary_entries.is_empty(),
            "Finds a dictionary entry when one is present and appends it to the return data.");

    assert!(res.content.contains("<DictionaryEntry"), "Conundrum parses DictionaryEntry components");
    assert!(res.content.contains("<NoteLink"), "Conundrum parses NoteLink components");
    assert!(res.content.contains("<FlusterCitation"), "Conundrum parses FlusterCitation components");
    assert!(res.content.contains("<AutoInsertedTag"), "Conundrum parses AutoInsertedTag components");

    assert!(res.content
               .contains(&format!("<{}", &AutoInsertedComponentName::FlusterAiParsePendingContainer.to_string())),
            "Conundrum parses Ai Parsing Request components");

    assert!(res.outgoing_links.iter().any(|x| x.link_to_note_id == "testNoteId"), "Finds nested outgoing links.");

    println!("{}", res.content);

    // assert!(res.content.contains("<FlusterCitation"))

    // insta::assert_snapshot!(&res.content);
}

#[tokio::test]
async fn conundrum_parses_documentation_requests() {
    for name in EmbeddableComponentName::iter() {
        let short_test_content = format!("# My markdown\n\n{}?", name);
        let full_test_content = format!("# My markdown\n\n{}??", name);

        let short_opts = ParseConundrumOptions { note_id: None,
                                                 content: short_test_content,
                                                 modifiers: Vec::new(),

                                                 hide_components: Vec::new() };

        let short_res =
            run_conundrum(short_opts).await.expect("Returns a vald result when a valid input was provided.");

        let full_opts = ParseConundrumOptions { note_id: None,
                                                content: full_test_content,
                                                modifiers: Vec::new(),
                                                hide_components: Vec::new() };

        let full_res = run_conundrum(full_opts).await.expect("Returns a vald result when a valid input was provided.");

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

        let short_opts = ParseConundrumOptions { note_id: None,
                                                 content: short_test_content,
                                                 modifiers: Vec::new(),
                                                 hide_components: Vec::new() };

        let short_res =
            run_conundrum(short_opts).await.expect("Returns a vald result when a valid input was provided.");

        let full_opts = ParseConundrumOptions { note_id: None,
                                                content: full_test_content,
                                                modifiers: Vec::new(),
                                                hide_components: Vec::new() };

        let full_res = run_conundrum(full_opts).await.expect("Returns a vald result when a valid input was provided.");

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
