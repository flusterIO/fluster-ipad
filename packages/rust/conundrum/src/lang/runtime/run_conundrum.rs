use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    lang::runtime::{
        parse_conundrum_string::parse_conundrum_string,
        traits::mdx_component_result::MdxComponentResult,
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

impl ParseMdxOptions {
    pub fn new(
        note_id: Option<String>,
        citations: Vec<CitationSummaryData>,
        content: String,
    ) -> Self {
        ParseMdxOptions {
            note_id,
            content,
            citations,
        }
    }
}

pub async fn run_conundrum(opts: ParseMdxOptions) -> MdxParsingResult {
    let mut result = MdxParsingResult::from_initial_mdx_content(&opts.content);
    result.note_id = opts.note_id.clone();
    let mut content = opts.content.as_str();

    let mut final_mdx = String::new();

    // The parser now handles the entire string in one go thanks to many0
    match parse_conundrum_string(&mut content) {
        Ok(elements) => {
            for element in elements {
                final_mdx.push_str(&element.to_mdx_component(&mut result));
            }
        }
        Err(_) => {
            println!("Error up here");
            // This should technically be unreachable now due to the anychar fallback,
            // but we keep it as a safety measure.
            final_mdx.push_str(&opts.content);
        }
    }

    result.content = final_mdx;
    result
}
