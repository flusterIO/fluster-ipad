use conundrum::{
    lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum},
    testing::render_sample,
};

#[tokio::test]
pub async fn renders_simple_task_list() {
    let test_content = r#"
- [x] Built-in Snippets for every component
    - [x] Context aware snippets (Math snippets that only appear where math is available, etc.)
- [x] Syntax highlighting
    - This actually currently uses the mdx syntax highlighter. Because mdx and conundrum, _currently_ are so close in syntax there is not much of a distinction, but Conundrum will provide it's own unique, feature-complete syntax highlighter in the coming months.
- [x] Embedded documentation, available completely offline built directly into the language itself.
- [ ] LSP driven intellisense.
    - This feature might be new to those that aren't developers, but let me tell you... it's a _game changer_ for productivity and speed. LSP stands for 'language-server-protocol', and it's just an application that runs in the background that applications like Fluster can ask for different types of responses based on your current cursor position, and with those responses we can do everything from autocomplete component properties to grammar checking sentences. This feature is just now being undertaken, so it will be a few months, but when this LSP is complete it will take writing Conundrum to a whole other level.
        "#;
    let res = run_conundrum(
           ParseConundrumOptions {
               content: test_content.to_string(),
               note_id: None,
               modifiers: Vec::new(),
               hide_components: Vec::new(),
               ..Default::default()
           }
       ).expect("Runs conundrum without throwing an error.");
    insta::assert_snapshot!(res.content);
}
