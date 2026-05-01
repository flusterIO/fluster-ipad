use conundrum::{
    lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum},
    testing::render_test::render_test,
};

#[tokio::test]
async fn parses_footnotes() {
    let test_content = r#"# My title here

Consectetur auctor hac lectus ex, a, natoque maecenas arcu pulvinar. Aenean, arcu ac sed nunc adipiscing, in sit laoreet eget. Mus non, elit dui, nascetur cras diam est ac in. Velit, nec, pellentesque suscipit at laoreet suspendisse auctor tempor sed. Porttitor nisi convallis gravida elementum quis consectetur, integer elit tempus.[^1]

Ullamcorper congue hac rhoncus consectetur, est urna arcu dui amet. Rhoncus lectus aenean consequat consectetur platea aenean et convallis, auctor. Nisi at convallis convallis, molestie pharetra eget vel lectus aliquam. Velit quis ut elit suspendisse vulputate, ligula nibh libero efficitur. Nisl metus vulputate, posuere, consectetur elit lorem euismod parturient dui.

Velit sit suscipit natoque tortor nunc velit metus ut faucibus. Mus eget, eget, vitae tincidunt nisi elementum id convallis nisi. Vestibulum pretium eu nibh ac eget nunc at, eget, felis. Vel, tristique aliquam condimentum, quis posuere aliquet felis at ut. Dignissim lacus elit non orci faucibus ultricies diam tortor sed.

A ante iaculis nisl praesent, vestibulum velit lectus nulla ac. Et at, pretium, bibendum dictumst a donec dictum bibendum aliquet. Congue leo dictumst lectus, eu sit nunc nullam, non condimentum. Eleifend phasellus nibh neque pellentesque, bibendum ante eget aenean sit. Dui eleifend ac placerat et, neque rutrum, diam lorem facilisis.

Consequat rutrum nam eu nisl quis, dignissim nunc in luctus. Dignissim nisl, lacus diam at, nunc eu est faucibus lorem. Lectus vitae, vitae rhoncus, ut et pellentesque tellus ultricies dictumst. Eu suspendisse praesent ut elementum, sem, dui lectus aenean laoreet. Ut, tempor dui ante eget sit phasellus, efficitur libero id.

[^1]: This is footnote one. Nec ante urna vitae arcu risus parturient, lectus est arcu. Convallis ipsum tristique in, dictumst, sit nam interdum natoque a. Tempus eget vestibulum vitae praesent feugiat posuere, porttitor varius rhoncus. Congue, cras dapibus aenean pulvinar tincidunt tristique, pharetra sed eleifend. Vulputate iaculis dolor magna ut eleifend id amet cursus bibendum. 

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
