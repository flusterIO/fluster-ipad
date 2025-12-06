use async_trait::async_trait;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use regex::Regex;

use crate::{
    parse::by_regex::{
        parse_mdx_by_regex::ParseMdxOptions,
        regex_parsers::mdx_parser::{MdxParser, ParserId},
    },
    parsing_result::{citation_result::CitationResult, mdx_parsing_result::MdxParsingResult},
};

pub struct CitationRegexParser;

#[async_trait]
impl MdxParser for CitationRegexParser {
    fn parser_id(&self) -> ParserId {
        ParserId::Citations
    }
    async fn parse_async(&self, req: &ParseMdxOptions, result: &mut MdxParsingResult) {
        let r = Regex::new(r#"\[\[cite:(?<citation_id>[^\]]+)\]\]"#)
            .expect("Creates regex without throwing an error.");

        let mut new_content = String::from(&result.content);
        let mut citations: Vec<CitationResult> = Vec::new();
        for result in r.captures_iter(&result.content) {
            if let Some(found_citation_key) = result.get(1) {
                let citation_key_as_string = found_citation_key.as_str();
                if !citations
                    .par_iter()
                    .any(|citation_item| citation_item.citation_key == citation_key_as_string)
                {
                    if let Some(citation_result) =
                        CitationResult::new(citation_key_as_string, &req.citations)
                    {
                        citations.push(citation_result);
                        let x = result
                            .get(0)
                            .expect("Must have at least the first match group.")
                            .as_str();
                        new_content = new_content.replace(
                            x,
                            &format!(
                                "<FlusterCitation citationKey='{}' index={{{}}} >",
                                citation_key_as_string,
                                citations.len()
                            ),
                        );
                    }
                }
            }
        }
        result.content = new_content;
        result.citations = citations;
    }
}
