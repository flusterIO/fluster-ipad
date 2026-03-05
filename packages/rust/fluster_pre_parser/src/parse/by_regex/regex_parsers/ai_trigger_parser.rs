use crate::{
    parse::by_regex::{
        parse_mdx_by_regex::ParseMdxOptions,
        regex_parsers::{
            mdx_parser::{MdxParser, ParserId},
            utility_parsers::code_block_parser::CodeBlockParser,
        },
    },
    parsing_result::{
        ai_serialization_request::AiSerializationRequestPhase1,
        mdx_parsing_result::MdxParsingResult,
    },
};
use async_trait::async_trait;
use fluster_core_utilities::core_types::component_constants::auto_inserted_component_name::AutoInsertedComponentName;

pub struct AiTriggerParser;

#[async_trait]
impl MdxParser for AiTriggerParser {
    fn parser_id(&self) -> ParserId {
        ParserId::AiTrigger
    }
    async fn parse_async(&self, _: &ParseMdxOptions, result: &mut MdxParsingResult) {
        let res = CodeBlockParser::parse_and_replace(&mut result.content, "fluster-ai", |c| {
            format!(
                "<{} stringifiedResult={}>\n{}\n</{}>",
                AutoInsertedComponentName::FlusterAiParsePendingContainer,
                serde_json::to_string(c)
                    .map(|x| format!("{{'{}'}}", x))
                    .unwrap_or("{null}".to_string()),
                c.block_content,
                AutoInsertedComponentName::FlusterAiParsePendingContainer
            )
        })
        .await;

        for code_block_result in res {
            result
                .ai_secondary_parse_requests
                .push(AiSerializationRequestPhase1 {
                    parsing_result: code_block_result,
                });
        }
    }
}

#[cfg(test)]
mod tests {

    use std::ops::Index;

    use super::*;

    #[tokio::test]
    async fn parses_ai_code_blocks_properly() {
        let opts = ParseMdxOptions {
            citations: Vec::new(),
            note_id: None,
            content: r#"# My note

````fluster-ai
Can you help me summarize this note please?
````
            "#
            .to_string(),
        };
        let mut initial_result = MdxParsingResult::from_initial_mdx_content(&opts.content.clone());

        AiTriggerParser {}
            .parse_async(&opts, &mut initial_result)
            .await;

        assert!(
            initial_result.ai_secondary_parse_requests.len() == 1,
            "Finds 1 result when 1 AI code block is present."
        );
        insta::assert_snapshot!(initial_result.content);
    }
}
