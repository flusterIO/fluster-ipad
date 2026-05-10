use conundrum::{
    lang::runtime::{
        run_conundrum::{ParseConundrumOptions, run_conundrum},
        state::{
            parse_state::{ConundrumCompileTarget, ConundrumModifier},
            ui_params::UIParams,
        },
    },
    parsers::{
        conundrum::logic::number::conundrum_int::ConundrumInt,
        markdown::code_block::supported_themes::SupportedCodeBlockTheme,
    },
    testing::{render_test::render_test, test_result::TestResult},
};

use crate::render_tests;

#[tokio::test]
async fn parses_footnote_footer_with_trailing_empty_space() {
    let test_content = r#"
My content here[^1].

[^1]: This is footnote one. Nec ante urna vitae arcu risus parturient, lectus est arcu. Convallis ipsum tristique in, dictumst, sit nam interdum natoque a. Tempus eget vestibulum vitae praesent feugiat posuere, porttitor varius rhoncus. Congue, cras dapibus aenean pulvinar tincidunt tristique, pharetra sed eleifend. Vulputate iaculis dolor magna ut eleifend id amet cursus bibendum.

"#;

    let res = run_conundrum(ParseConundrumOptions { note_id: None,
        content: test_content.to_string(),
        hide_components: Vec::new(),
        modifiers: Vec::new(),
        ..Default::default()
    }).inspect_err(|e| {
        println!("Error: {:#?}", e);
    }).expect("Returns a vald result when a valid input was provided.");

    println!("Res: {:#?}", res);
    assert!(res.footnotes.contains_key(&ConundrumInt(1)), "Found the footnote as expected.");
}

#[tokio::test]
async fn parses_footnotes() {
    let test_content = r#"# My title here

Consectetur auctor hac lectus ex, a, natoque maecenas arcu pulvinar. Aenean, arcu ac sed nunc adipiscing, in sit laoreet eget. Mus non, elit dui, nascetur cras diam est ac in. Velit, nec, pellentesque suscipit at laoreet suspendisse auctor tempor sed. Porttitor nisi convallis gravida elementum quis consectetur, integer elit tempus.[^2]

Ullamcorper congue hac rhoncus consectetur, est urna arcu dui amet. Rhoncus lectus aenean consequat consectetur platea aenean et convallis, auctor. Nisi at convallis convallis, molestie pharetra eget vel lectus aliquam. Velit quis ut elit suspendisse vulputate, ligula nibh libero efficitur. Nisl metus vulputate, posuere, consectetur elit lorem euismod parturient dui.[^1]

Velit sit suscipit natoque tortor nunc velit metus ut faucibus. Mus eget, eget, vitae tincidunt nisi elementum id convallis nisi. Vestibulum pretium eu nibh ac eget nunc at, eget, felis. Vel, tristique aliquam condimentum, quis posuere aliquet felis at ut. Dignissim lacus elit non orci faucibus ultricies diam tortor sed.

A ante iaculis nisl praesent, vestibulum velit lectus nulla ac. Et at, pretium, bibendum dictumst a donec dictum bibendum aliquet. Congue leo dictumst lectus, eu sit nunc nullam, non condimentum. Eleifend phasellus nibh neque pellentesque, bibendum ante eget aenean sit. Dui eleifend ac placerat et, neque rutrum, diam lorem facilisis.

Consequat rutrum nam eu nisl quis, dignissim nunc in luctus. Dignissim nisl, lacus diam at, nunc eu est faucibus lorem. Lectus vitae, vitae rhoncus, ut et pellentesque tellus ultricies dictumst. Eu suspendisse praesent ut elementum, sem, dui lectus aenean laoreet. Ut, tempor dui ante eget sit phasellus, efficitur libero id.

[^1]: This is footnote uno. Nec ante urna vitae arcu risus parturient, lectus est arcu. Convallis ipsum tristique in, dictumst, sit nam interdum natoque a. Tempus eget vestibulum vitae praesent feugiat posuere, porttitor varius rhoncus. Congue, cras dapibus aenean pulvinar tincidunt tristique, pharetra sed eleifend. Vulputate iaculis dolor magna ut eleifend id amet cursus bibendum.

[^2]: This is footnote two. Nec ante urna vitae arcu risus parturient, lectus est arcu. Convallis ipsum tristique in, dictumst, sit nam interdum natoque a. Tempus eget vestibulum vitae praesent feugiat posuere, porttitor varius rhoncus. Congue, cras dapibus aenean pulvinar tincidunt tristique, pharetra sed eleifend. Vulputate iaculis dolor magna ut eleifend id amet cursus bibendum.

"#;

    // let res = run_conundrum(ParseConundrumOptions { note_id: None,
    //     content: test_content.to_string(),
    //     hide_components: Vec::new(),
    //     modifiers: Vec::new(),
    //     ..Default::default()
    // }).expect("Returns a vald result when a valid input was provided.");

    let _ = render_test(test_content, "simple-footnotes").await;
    // insta::assert_snapshot!(res.content);
}

#[tokio::test]
async fn parses_footnotes_from_paper() {
    let test_content = r#"# The Lighthouse & The Clocktower
> A sample note & a novel interpretation of relativity

<Admonition title="Thank you" foldable folded>
Thank you for taking the time to read this. My story is posted elsewhere, so to keep things short, this is the entire reason I built Fluster in the first place.[^1]

Almost 5 years ago I realized Einstein made an assumption that made far more sense before the observations that give us the Big Bang, and after playing around with the math I realized I can derive an absolutely staggering amount of physics from a single, mathematically equivalent modification to Einstein's model. I quit my job, became homeless, and over the course of this pursuit built the original version of Fluster for my own personal use.

I'll do my best to convince the math types among you by the end of this sample note that Einstein made a mistake, but regardless of your opinions on the model, I hope you enjoy Fluster and find it as useful as I have.

</Admonition>

Today in class we learned about relativistic simultaneity, but something just doesn't seem right.

For starts, what happens if


[^1]: The logical question here is well, why not just publish it in peer review? 
       "#;

    let test_input = ParseConundrumOptions { content: test_content.to_string(),
                                             hide_components: Vec::new(),
                                             modifiers: Vec::new(),
                                             note_id: None,
                                             target: ConundrumCompileTarget::Html,
                                             trusted: true,
                                             ui_params: UIParams { dark_mode: true,
                                                                   font_scalar: 1.0,
                                                                   math_font_scalar: 1.2,
                                                                   syntax_theme:
                                                                       Some(SupportedCodeBlockTheme::Darkneon) } };
    let r = run_conundrum(test_input).inspect_err(|e| {
                                         println!("Error: {:#?}", e);
                                     })
                                     .expect("Failed to parse the footnotes from paper test.");

    let t = TestResult(r);

    t.log_html();

    assert!(t.count_css_query("div.cdrm-footnote-body") == 1, "Finds the proper number of rendered footnote elements");
}
