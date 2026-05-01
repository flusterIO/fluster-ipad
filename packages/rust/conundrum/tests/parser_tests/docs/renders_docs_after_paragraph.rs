use conundrum::lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum};

#[tokio::test]
pub async fn renders_docs_after_paragraph() {
    let test_content = r#"
# Welcome to Fluster!
> The _extended_ markdown application for modern students & academics.

$$ 
\text{Inertia}\rightarrow \delta_{(R)} = \frac{1}{R} \int_0^R 2G \frac{M_\oplus}{R_\oplus^3} \hat{R} = 526.6\scriptstyle\frac{\text{km}}{\text{s}}
$$


If you're familiar with markdown, you know that ^ shouldn't be a subtitle, but a block quote. You're right, but in Fluster, there are a few additional features, including a title/block-quote pair on 2 lines directly next to each other results in a subtitle... it even appears in your note's search results as it's description!
  
See this `Docs??` below? That's a documentation tag. Enter one of those on a line all by itself at any point to show the Conundrum transpiler's built-in documentation. 1 `?` for the short-ish documentation, 2 `??` for the full documentation.  

Docs??
        "#;
    let res = run_conundrum(ParseConundrumOptions {
        content: test_content.to_string(),
        ..Default::default()
    }).expect("Parses content without throwing an error.");

    println!("Result: {}", res.content);
}
