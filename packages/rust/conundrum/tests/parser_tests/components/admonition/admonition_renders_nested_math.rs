#[cfg(test)]
mod tests {

    use conundrum::lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum};
    use insta::assert_snapshot;

    #[tokio::test]
    async fn admonition_renders_nested_math() {
        let input = r#"
# My title
> Lorem ipsum, I don't remember the poem..

$$
\Delta T = m
$$


<Admonition title="My admonition">
  $$
  e=mc^2
  $$
</Admonition>
"#;

        let res = run_conundrum(ParseConundrumOptions { content: input.to_string(),
        note_id: None,
        hide_components: Vec::new(),
        modifiers: Vec::new(),
        ..Default::default()
        }).expect("Returns a vald result when a valid input was provided.");

        assert_snapshot!(res.content)
    }
}
