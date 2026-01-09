use async_trait::async_trait;
use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    parse::by_regex::{
        parse_mdx_by_regex::ParseMdxOptions,
        regex_parsers::mdx_parser::{MdxParser, ParserId},
    },
    parsing_result::mdx_parsing_result::MdxParsingResult,
};

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct NoteOutgoingLinkResult {
    /// The user defined id on the target note.
    pub link_to_note_id: String,
}

pub struct NoteLinkRegexParser;

#[async_trait]
impl MdxParser for NoteLinkRegexParser {
    fn parser_id(&self) -> ParserId {
        ParserId::NoteLink
    }
    async fn parse_async(&self, _: &ParseMdxOptions, result: &mut MdxParsingResult) {
        let r = Regex::new(r"\[([^\]]+)\]\(note:(\S+)\)").unwrap();
        let mut new_content = String::from(&result.content);
        let mut outgoing_links: Vec<NoteOutgoingLinkResult> = Vec::new();
        for result in r.captures_iter(&result.content) {
            if let (Some(body), Some(note_id)) = (result.get(1), result.get(2)) {
                let body_as_string = body.as_str();
                let note_id_as_string = note_id.as_str();
                let complete_match = result.get(0).unwrap().as_str();
                if !outgoing_links
                    .par_iter()
                    .any(|tag_item| tag_item.link_to_note_id == note_id_as_string)
                {
                    outgoing_links.push(NoteOutgoingLinkResult {
                        link_to_note_id: note_id_as_string.to_string(),
                    });
                    new_content = new_content.replace(
                        complete_match,
                        &format!(
                            "<NoteLink id=\"{}\">{}</NoteLink>",
                            note_id_as_string, body_as_string
                        ),
                    );
                }
            }
        }
        result.content = new_content;
        result.outgoing_links = outgoing_links;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn parses_note_outgoing_links_properly() {
        let opts = ParseMdxOptions {
            citations: Vec::new(),
            note_id: None,
            content: r#"# My note

This is [my note link](note:myNoteId) here.
            "#
            .to_string(),
        };
        let mut initial_result = MdxParsingResult::from_initial_mdx_content(&opts.content.clone());

        NoteLinkRegexParser {}
            .parse_async(&opts, &mut initial_result)
            .await;

        let should_equal = r#"# My note

This is <NoteLink id="myNoteId">my note link</NoteLink> here.
            "#;

        assert_eq!(
            initial_result.content, should_equal,
            "Parses note outgoing link to mdx string as expected."
        )
    }
}
