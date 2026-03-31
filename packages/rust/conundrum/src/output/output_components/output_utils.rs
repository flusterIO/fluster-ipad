pub fn format_embedded_object_property(item: String) -> String {
    format!("{{{}}}", item)
}

pub fn javascript_null_prop() -> String {
    "{null}".to_string()
}

pub fn format_markdown_fragment_property(item: &str) -> String {
    format!("{{<>{}</>}}", item)
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn formats_markdown_fragment_properly() {
        let res = format_markdown_fragment_property("## My Markdown here!");
        assert_snapshot!(res);
    }
}
