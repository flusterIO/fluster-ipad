use cli_table::format;
use conundrum::testing::render_test::render_test;
use indoc::indoc;

#[tokio::test]
async fn renders_colors_to_html() {
    let test_content = indoc! {"
<Color color=\"#f00\" />

<Color error />
        
        ",

    };
    render_test(test_content, "ColorComponent").await;
}
