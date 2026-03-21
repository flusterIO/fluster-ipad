use std::cell::RefCell;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    lang::runtime::{
        compile_conundrum::compile_elements, parse_conundrum_string::parse_conundrum_string,
        state::parse_state::ParseState,
    },
    output::parsing_result::{citation_result::CitationSummaryData, mdx_parsing_result::MdxParsingResult},
};
use winnow::Stateful;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, uniffi::Record, Clone)]
pub struct ParseMdxOptions {
    pub note_id: Option<String>,
    pub content: String,
    pub citations: Vec<CitationSummaryData>,
}

impl ParseMdxOptions {
    pub fn new(note_id: Option<String>, citations: Vec<CitationSummaryData>, content: String) -> Self {
        ParseMdxOptions { note_id,
                          content,
                          citations }
    }
}

pub async fn run_conundrum(opts: ParseMdxOptions) -> MdxParsingResult {
    let mut result = MdxParsingResult::from_initial_mdx_content(&opts.content);
    result.note_id = opts.note_id.clone();
    let state = RefCell::new(ParseState { data: MdxParsingResult::from_initial_mdx_content(&opts.content) });
    let mut stateful_input = Stateful { input: opts.content.as_str(),
                                        state };

    let final_mdx = match parse_conundrum_string(&mut stateful_input) {
        Ok(elements) => compile_elements(&elements, &mut result),
        Err(err) => {
            println!("Parser Error: {:#?}", err);
            opts.content.clone()
        }
    };

    result.content = final_mdx;
    result
}
