use async_trait::async_trait;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    parse::{
        by_regex::{
            parse_mdx_by_regex::ParseMdxOptions,
            regex_parsers::mdx_parser::{MdxParser, ParserId},
        },
        utils::format_javascript_string::format_javascript_string,
    },
    parsing_result::mdx_parsing_result::MdxParsingResult,
};

pub struct DictionaryEntryRegexParser;

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct DictionaryEntryResult {
    /// Primary Key
    pub label: String,
    pub body: String,
}

#[async_trait]
impl MdxParser for DictionaryEntryRegexParser {
    fn parser_id(&self) -> ParserId {
        ParserId::Dictionary
    }

    async fn parse_async(&self, _: &ParseMdxOptions, result: &mut MdxParsingResult) {
        let r = Regex::new(r#"```dictionary\s+--\s?(?P<title>[^\n]+)\n(?P<body>[^`]+)\n```"#)
            .expect("Creates regular expression without throwing an error.");

        let note_id = result
            .note_id
            .as_ref()
            .map(|x| {
                println!("Id: {}", x);
                format!("\"{}\"", format_javascript_string(x))
            })
            .unwrap_or_else(|| "{undefined}".to_string());
        let mut results: Vec<DictionaryEntryResult> = Vec::new();
        let mut new_content = String::from(&result.content);
        for capture_result in r.captures_iter(&result.content) {
            let complete_match = capture_result.get(0);
            let body_match = capture_result.get(2);
            let title_match = capture_result.get(1);
            if let (Some(body), Some(title)) = (body_match, title_match) {
                results.push(DictionaryEntryResult {
                    label: title.as_str().to_string(),
                    body: body.as_str().to_string(),
                });
                new_content = new_content.replace(
                    complete_match.unwrap().as_str(),
                    &format!(
                        r#"<DictionaryEntry label="{}" noteId={} >
{}
</DictionaryEntry>"#,
                        format_javascript_string(title_match.unwrap().as_str()),
                        note_id,
                        body_match.unwrap().as_str()
                    ),
                );
            }
        }
        result.dictionary_entries = results;
        result.content = new_content
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn parses_dictionary_properly() {
        let opts = ParseMdxOptions {
            citations: Vec::new(),
            note_id: Some("test-note-id".to_string()),
            content: r#"# My note

```dictionary -- This is my dictionary title.
This is my dictionary entry body.
```
            "#
            .to_string(),
        };

        let mut initial_result = MdxParsingResult::from_initial_mdx_content(&opts.content.clone());

        initial_result.note_id = opts.note_id.clone();

        DictionaryEntryRegexParser {}
            .parse_async(&opts, &mut initial_result)
            .await;

        assert!(
            initial_result.dictionary_entries.len() == 1
                && initial_result
                    .dictionary_entries
                    .first()
                    .expect("Finds first element")
                    .label
                    == "This is my dictionary title.",
            "Tag parser finds second result."
        );

        let should_equal = r#"# My note

<DictionaryEntry label="This is my dictionary title." noteId="test-note-id" >
This is my dictionary entry body.
</DictionaryEntry>
            "#;

        assert_eq!(
            initial_result.content, should_equal,
            "Parses dictionary entries to mdx string as expected."
        )
    }
}
