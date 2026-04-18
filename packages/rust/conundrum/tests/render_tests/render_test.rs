use conundrum::{
    lang::runtime::{
        run_conundrum::{ParseConundrumOptions, run_conundrum},
        state::{
            parse_state::{ConundrumCompileTarget, ConundrumModifier},
            ui_params::UIParams,
        },
    },
    parsers::markdown::code_block::supported_themes::SupportedCodeBlockTheme,
    testing::{
        render_sample::{self, write_test_sandbox_output_for_dev},
        test_result::TestResult,
    },
};

pub async fn render_test(content: &str, label: &str) -> TestResult {
    let test_input = ParseConundrumOptions { content: content.to_string(),
                                             hide_components: Vec::new(),
                                             modifiers: vec![ConundrumModifier::Standalone],
                                             note_id: None,
                                             target: ConundrumCompileTarget::Html,
                                             trusted: true,
                                             ui_params: UIParams { dark_mode: true,
                                                                   font_scalar: 1.0,
                                                                   math_font_scalar: 1.2,
                                                                   syntax_theme:
                                                                       Some(SupportedCodeBlockTheme::SolarizedDark) } };
    let r = run_conundrum(test_input).await.expect(format!("Failed to parse the `{}` test.", label).as_str());
    println!("R: {:#?}", r.content);

    // NOTE: Toggle this off for ui development but remember to turn it back on!
    // insta::assert_snapshot!(label, r.content);
    //

    write_test_sandbox_output_for_dev(r.content.as_str());
    TestResult(r)
}
