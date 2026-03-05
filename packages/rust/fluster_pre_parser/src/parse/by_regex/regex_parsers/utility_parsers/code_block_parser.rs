use fancy_regex::Regex;
use fluster_core_utilities::core_types::ai::CodeBlockParsingResult;
pub struct CodeBlockParser {}

impl CodeBlockParser {
    /// This regular expression is no longer linear due to the lookbehind, but is required to
    /// match the varable depth of the input codeblock.
    /// For performance reasons, prefer other parsers where possible.
    fn get_regex() -> Regex {
        Regex::new(r"(?:^|\n)(`{3,})(.*?)\n([\s\S]*?)\n\1").unwrap()
    }

    pub async fn parse_and_replace(
        content: &mut String,
        language_key: &str,
        replacer: impl Fn(&CodeBlockParsingResult) -> String,
    ) -> Vec<CodeBlockParsingResult> {
        let results = CodeBlockParser::parse_async(content, language_key).await;
        for code_block_result in &results {
            let replace_with = replacer(code_block_result);
            *content = content.replace(&code_block_result.full_match, &replace_with);
        }
        results
    }
    pub async fn parse_async(content: &str, language_key: &str) -> Vec<CodeBlockParsingResult> {
        let re = CodeBlockParser::get_regex();
        let mut results: Vec<CodeBlockParsingResult> = Vec::new();
        for captures in re.captures_iter(content).flatten() {
            let full_match = captures.get(0).map_or("", |m| m.as_str());
            let language = captures.get(2).map_or("", |m| m.as_str());
            let code_content = captures.get(3).map_or("", |m| m.as_str());
            if language == language_key {
                results.push(CodeBlockParsingResult {
                    full_match: full_match.to_string(),
                    language_tag: language.to_string(),
                    block_content: code_content.to_string(),
                });
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Index;

    use insta::assert_snapshot;

    use super::*;

    #[tokio::test]
    async fn code_block_parser() {
        let test_input_with_block = r#"
````fluster-ai
# Help me

Can you please generate a summary of this note.
````
"#;
        let res = CodeBlockParser::parse_async(test_input_with_block, "fluster-ai").await;
        assert!(res.len() == 1, "Finds one result when one is present.");
        let item = res.index(0);
        assert!(
            item.language_tag == "fluster-ai",
            "Finds the fluster-ai tag."
        );
        assert_snapshot!(item.block_content);
        // assert_eq!(result, 4);
    }
}
