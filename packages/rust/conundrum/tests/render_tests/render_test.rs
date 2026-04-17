use conundrum::{
    lang::runtime::{
        run_conundrum::{ParseConundrumOptions, run_conundrum},
        state::ui_params::UIParams,
    },
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
    parsers::markdown::code_block::supported_themes::SupportedCodeBlockTheme,
};

pub async fn render_test(content: &str, label: &str) -> MdxParsingResult {
    let test_input = ParseConundrumOptions { content: content.to_string(),
                                             hide_components: Vec::new(),
                                             modifiers: Vec::new(),
                                             note_id: None,
                                             ui_params: UIParams { dark_mode: true,
                                                                   font_scalar: 1.0,
                                                                   math_font_scalar: 1.2,
                                                                   syntax_theme:
                                                                       Some(SupportedCodeBlockTheme::SolarizedDark) } };
    let r = run_conundrum(test_input).await.expect(format!("Failed to parse the `{}` test.", label).as_str());
    println!("R: {:#?}", r.content);
    insta::assert_snapshot!(label, r.content);
    r
}
