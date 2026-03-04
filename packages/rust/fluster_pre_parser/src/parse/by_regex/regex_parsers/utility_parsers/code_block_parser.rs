use fancy_regex::Regex;
pub struct CodeBlockParser {}

pub struct CodeBlockParsingResult {
    pub full_match: String,
    pub langage_tag: String,
    pub block_content: String,
}

impl CodeBlockParser {
    /// This regular expression is no longer linear due to the lookbehind, but is required to
    /// match the varable depth of the input codeblock.
    /// For performance reasons, prefer other parsers where possible.
    fn get_regex(&self) -> Regex {
        Regex::new(r"(?:^|\n)(`{3,})(.*?)\n([\s\S]*?)\n\1").unwrap()
    }
    pub async fn parse_async(&self, content: &str) -> Vec<CodeBlockParsingResult> {
        let re = self.get_regex();
        let mut results: Vec<CodeBlockParsingResult> = Vec::new();
        for captures in re.captures_iter(content).flatten() {
            let full_match = captures.get(0).map_or("", |m| m.as_str());
            // Handle the Result returned by fancy_regex
            // Group 1: The backticks
            let backticks = captures.get(1).map_or("", |m| m.as_str());

            // Group 2: The language identifier
            let language = captures.get(2).map_or("", |m| m.as_str());

            // Group 3: The code block content
            let code_content = captures.get(3).map_or("", |m| m.as_str());

            println!("--- Found Code Block ---");
            println!("Found full match: {}", full_match);
            println!("Backticks: {}", backticks);
            println!("Language:  {}", language);
            println!("Content:\n{}", code_content);
            results.push(CodeBlockParsingResult {
                full_match: full_match.to_string(),
                langage_tag: language.to_string(),
                block_content: code_content.to_string(),
            });
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn code_block_parser() {
        let test_input_with_block = r#"
````fluster-ai
# Help me

Can you please generate a summary of this note.
````
"#;
        let cb = CodeBlockParser {};
        let res = cb.parse_async(test_input_with_block).await;
        assert!(res.len() == 1, "Finds one result when one is present.");
        // assert_eq!(result, 4);
    }
}
