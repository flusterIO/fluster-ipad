use conundrum::{
    lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum},
    testing::render_test::render_test,
};

#[tokio::test]
pub async fn renders_simple_ordered_list() {
    let test_content = r#"
1. Tag
  - Add as many tags to each note as you like using the `[[#myTag]]` syntax. These are automatically picked up by the Conundrum transpiler, linked to your note, and made searchable throughout your application.
2. Topics & Subjects
  - A 'topic' and a 'subject' are basically identical in functionality to the tag, apart from that each note can have at most 1 topic and 1 subject. I'd recommend using these in a sort of hierarchic way, with one being more broad than the other. For a high school student this might look like a set of subjects of 'math', 'physics', 'personal' and a set of topics like 'complex-numbers', 'Newtonian-gravity', and 'finance'.
3. Dictionary Entries
  - Conundrum includes support for embedding dictionary entries directly in your notes and exposing them to a larger 'dictionary' page which can be used to link back to the note where the dictionary entry was created.
4. Citations 
  - Even if you're not into academic note taking specifically, this tool is something that you should consider giving a try. By making notes searchable by citation, we can support _any_ link type that is supported by BibLatex. You can cite websites (even specific social media channels), books, movies... even events you attended & conversations you had! There are a set of built in snippets to help you complete these entries, and editor support for the Biblatex language to make inserting these citations more reliable.
    - Citations can be formatted by any of more than a dozen included citation formats, with support for arbitrary `.csl` files available in the coming months.
    - See the embedded `Syntax??` documentation for more information.
        "#;

    let res = run_conundrum(ParseConundrumOptions { note_id: None,
        content: test_content.to_string(),
        hide_components: Vec::new(),
        modifiers: Vec::new(),
        ..Default::default()
    }).expect("Returns a vald result when a valid input was provided.");

    let r = render_test(test_content, "ordered-list").await;

    // let p_count = r.count_css_query("p");
    // println!("R: {}", p_count);

    // assert!(p_count == 5, "Finds the correct number of paragraphs.");
}
