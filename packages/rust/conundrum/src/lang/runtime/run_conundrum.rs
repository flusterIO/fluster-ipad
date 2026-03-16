use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    lang::runtime::{
        parse_any_element::parse_any_element, traits::mdx_component_result::MdxComponentResult,
    },
    output::parsing_result::{
        citation_result::CitationSummaryData, mdx_parsing_result::MdxParsingResult,
    },
};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, uniffi::Record, Clone)]
pub struct ParseMdxOptions {
    pub note_id: Option<String>,
    pub content: String,
    pub citations: Vec<CitationSummaryData>,
}

pub async fn run_conundrum(opts: ParseMdxOptions) -> MdxParsingResult {
    let mut result = MdxParsingResult::from_initial_mdx_content(&opts.content);
    result.note_id = opts.note_id.clone();
    let mut final_mdx = String::new();
    let mut curr_input = opts.content.as_str();
    let mut markdown_buffer = String::new();

    while !curr_input.is_empty() {
        // Look for ANY of our custom syntaxes
        match parse_any_element(curr_input) {
            Ok((next_input, element)) => {
                // Flush the raw markdown text into our final output
                if !markdown_buffer.is_empty() {
                    final_mdx.push_str(&markdown_buffer);
                    markdown_buffer.clear();
                }

                final_mdx.push_str(&element.to_mdx_component(&mut result));

                curr_input = next_input;
            }
            Err(_) => {
                // If nothing matches, consume exactly one character into the buffer
                let mut chars = curr_input.chars();
                if let Some(c) = chars.next() {
                    markdown_buffer.push(c);
                    curr_input = chars.as_str();
                } else {
                    break;
                }
            }
        }
    }

    if !markdown_buffer.is_empty() {
        final_mdx.push_str(&markdown_buffer);
    }

    result.content = final_mdx;

    result
}
