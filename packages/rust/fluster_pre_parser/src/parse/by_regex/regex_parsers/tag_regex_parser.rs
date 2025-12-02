use async_trait::async_trait;
use rayon::prelude::*;
use regex::Regex;

use crate::{
    parse::by_regex::regex_parsers::mdx_parser::{MdxParser, ParserId},
    parsing_result::{mdx_parsing_result::MdxParsingResult, tag_result::TagResult},
};

pub struct TagRegexParser;

#[async_trait]
impl MdxParser for TagRegexParser {
    fn parser_id(&self) -> ParserId {
        ParserId::Tag
    }
    async fn parse_async(&self, result: &mut MdxParsingResult) {
        let r = Regex::new(r"\[\[#(?<body>[^\]]+)\]\]").unwrap();
        let mut new_content = String::from(&result.content);
        let mut tags: Vec<TagResult> = Vec::new();
        for result in r.captures_iter(&result.content) {
            if let Some(body) = result.get(1) {
                let body_as_string = body.as_str();
                if !tags
                    .par_iter()
                    .any(|tag_item| tag_item.body == body_as_string)
                {
                    tags.push(TagResult::new(body_as_string.to_string()));
                    new_content = new_content.replace(
                        &format!("[[#{}]]", body_as_string),
                        &format!("<AutoInsertedTag>{}</AutoInsertedTag>", body_as_string),
                    );
                }
            }
        }
        result.content = new_content;
        result.tags = tags;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn parses_tags_properly() {
        let mut initial_result = MdxParsingResult::from_initial_mdx_content(
            r#"
        # My note

        This is [[#my_tag]] and [[#myOtherTag]]
            "#,
        );

        TagRegexParser {}.parse_async(&mut initial_result).await;

        assert!(
            initial_result.tags.iter().any(|x| x.body == "myOtherTag"),
            "Tag parser finds second result."
        );

        let should_equal = r#"
        # My note

        This is <AutoInsertedTag>my_tag</AutoInsertedTag> and <AutoInsertedTag>myOtherTag</AutoInsertedTag>
            "#;

        assert_eq!(
            initial_result.content, should_equal,
            "Parses tags to mdx string as expected."
        )

        // assert_eq!(result, 4);
    }
}
