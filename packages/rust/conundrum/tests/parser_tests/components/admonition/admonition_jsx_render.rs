#[cfg(test)]
mod tests {

    use conundrum::lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum};
    use insta::assert_snapshot;

    #[tokio::test]
    async fn container_outputs_valid_jsx() {
        let input = r#"<Admonition title="My admonition's title" error folded foldable>
My content goes here.
</Admonition>"#;

        let res = run_conundrum(ParseConundrumOptions { content: input.to_string(),
                                                  note_id: None,
                                                  hide_components: Vec::new(),
                                                  modifiers: Vec::new() }).await.expect("Returns a vald result when a valid input was provided.");

        assert_snapshot!(res.content)
    }
}
