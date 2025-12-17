use async_trait::async_trait;
use chrono::Utc;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    parse::by_regex::{
        parse_mdx_by_regex::ParseMdxOptions,
        regex_parsers::mdx_parser::{MdxParser, ParserId},
    },
    parsing_result::mdx_parsing_result::MdxParsingResult,
};

pub struct DictionaryEntryRegexParser;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DictionaryEntryModelWithoutSource {
    /// Primary Key
    pub label: String,
    pub body: String,
    pub ctime: String,
}

#[async_trait]
impl MdxParser for DictionaryEntryRegexParser {
    fn parser_id(&self) -> ParserId {
        ParserId::Dictionary
    }

    async fn parse_async(&self, req: &ParseMdxOptions, result: &mut MdxParsingResult) {
        let r = Regex::new(r#"```dictionary\s+--\s?(?P<title>[^\n]+)\n(?P<body>[^`]+)\n```"#)
            .expect("Creates regular expression without throwing an error.");

        let mut results: Vec<DictionaryEntryModelWithoutSource> = Vec::new();
        let mut new_content = String::from(&result.content);
        let now = Utc::now().timestamp_millis().to_string();
        for result in r.captures_iter(&result.content) {
            let complete_match = result.get(0);
            let body_match = result.get(2);
            let title_match = result.get(1);
            if body_match.is_some() && title_match.is_some() {
                results.push(DictionaryEntryModelWithoutSource {
                    label: title_match.unwrap().as_str().to_string(),
                    body: body_match.unwrap().as_str().to_string(),
                    ctime: now.clone(),
                });
                new_content = new_content.replace(
                    complete_match.unwrap().as_str(),
                    &format!(
                        r#"<DictionaryEntry label='{}'>
{}
</DictionaryEntry>"#,
                        title_match.unwrap().as_str(),
                        body_match.unwrap().as_str()
                    ),
                );
            }
        }
        // RESUME: handle dictionary entry results here
    }
}
