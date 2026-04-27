use conundrum::{
    lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum},
    testing::{render_test::render_test, write_test_ast::write_test_ast},
};
use scraper::{Html, Selector};

#[tokio::test]
pub async fn renders_multiple_separated_paragraphs() {
    let test_content = r#"
Nullam elementum, imperdiet faucibus tincidunt, nulla pellentesque urna pretium est. Pharetra, imperdiet diam, eros pulvinar ipsum rhoncus eget odio tortor. Fermentum aliquam, odio non magna, tellus eget ut porttitor pellentesque. Luctus, a vulputate, malesuada massa tristique dapibus nulla arcu pulvinar. Cursus, condimentum bibendum bibendum, convallis praesent finibus fermentum vitae ultrices.

Metus elementum, suspendisse, eros nisi amet lorem ac ac dui. Bibendum odio cras faucibus sagittis imperdiet phasellus fringilla erat lacus. Convallis adipiscing a vulputate nullam laoreet consequat ac ac bibendum. Parturient sed tempus nam consectetur diam quis urna montes faucibus. Lacus, lacus auctor vitae dui dignissim ac, sed quisque porttitor.

Tortor bibendum, habitasse, euismod mi donec vel nunc eu eget. Pharetra metus lectus in mi praesent iaculis arcu vitae, in. Posuere ac, ligula, pretium condimentum nullam vulputate consectetur vulputate aliquam. Fermentum, placerat, pellentesque tincidunt est dis eu imperdiet dictum vel. Dignissim lectus efficitur neque, magna at tempor enim at fringilla.

Aliquam, purus, nunc neque arcu ipsum velit parturient orci suspendisse. Urna non tristique sit ante, laoreet, turpis nisl tortor consectetur. Ut eros diam, congue urna ut nulla, integer rutrum quam. Facilisis, arcu amet sit eleifend ut praesent rhoncus laoreet nullam. Nullam, convallis, ultrices ut feugiat eget ullamcorper purus volutpat dignissim.

Enim nunc, adipiscing eros ipsum eu eu massa, mi pharetra. Nulla hac lacus sagittis faucibus at nulla tristique in quisque. Pharetra arcu tempor tempus lectus justo ac risus dignissim ac. Vivamus, cursus praesent, amet ac ut nulla nam nascetur praesent. Consequat natoque imperdiet justo consequat sit finibus aenean urna id.
        "#;

    let res = run_conundrum(ParseConundrumOptions { note_id: None,
        content: test_content.to_string(),
        hide_components: Vec::new(),
        modifiers: Vec::new(),
        ..Default::default()
    }).expect("Returns a vald result when a valid input was provided.");

    let r = render_test(test_content, "multiple-paragraphs").await;

    let p_count = r.count_css_query("p");
    println!("R: {}", p_count);

    assert!(p_count == 5, "Finds the correct number of paragraphs.");

    insta::assert_snapshot!(res.content);
}
