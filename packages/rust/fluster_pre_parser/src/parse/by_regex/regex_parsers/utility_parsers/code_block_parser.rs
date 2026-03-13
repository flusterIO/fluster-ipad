use fancy_regex::Regex;
use fluster_core_utilities::core_types::ai::code_block_parsing_result::CodeBlockParsingResult;
pub struct CodeBlockParser {}

impl CodeBlockParser {
    /// This regular expression is no longer linear due to the lookbehind, but is required to
    /// match the varable depth of the input codeblock.
    /// For performance reasons, prefer other parsers where possible.
    fn get_regex() -> Regex {
        Regex::new(r"(?:^|\n)(`{3,})([^\-\n]*)(?:--\s*(.*?))?\n([\s\S]*?)\n\1").unwrap()
    }

    pub async fn parse_and_replace(
        content: &mut String,
        language_key: &str,
        replacer: impl Fn(&CodeBlockParsingResult) -> String,
    ) -> Vec<CodeBlockParsingResult> {
        let results = CodeBlockParser::parse_async(content, language_key).await;
        for code_block_result in &results {
            let replace_with = replacer(code_block_result);
            *content = content.replace(&code_block_result.get_full_match_rust(), &replace_with);
        }
        results
    }
    pub async fn parse_async(content: &str, language_key: &str) -> Vec<CodeBlockParsingResult> {
        let re = CodeBlockParser::get_regex();
        let mut results: Vec<CodeBlockParsingResult> = Vec::new();
        for captures in re.captures_iter(content).flatten() {
            let full_match = captures.get(0).map_or("", |m| m.as_str());
            let language = captures.get(2).map_or("", |m| m.as_str().trim());
            let meta_data = captures.get(3).map(|x| x.as_str().to_string());
            let code_content = captures.get(4).map_or("", |m| m.as_str());
            if language == language_key {
                results.push(CodeBlockParsingResult::new(
                    full_match.to_string(),
                    language.to_string(),
                    code_content.to_string(),
                    meta_data,
                ));
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
    async fn parses_codeblock_with_meta_data() {
        let test_input_with_block = r#"
````dictionary -- My dictionary tag
# Help me

Can you please generate a summary of this note.
````
"#;
        let res = CodeBlockParser::parse_async(test_input_with_block, "dictionary").await;
        assert!(res.len() == 1, "Finds one value when one value is present");
        let item = res.index(0);
        assert!(
            item.language_tag() == "dictionary",
            "Finds the proper language tag when meta data is present."
        );
        assert!(
            item.meta_data_rust() == Some("My dictionary tag".to_string()),
            "Finds the proper dictionary meta data"
        );
    }

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
            item.clone().get_lang_rust() == "fluster-ai",
            "Finds the fluster-ai tag."
        );
        assert_snapshot!(item.get_block_content_rust());
        // assert_eq!(result, 4);
    }
}
