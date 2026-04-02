use std::cell::RefCell;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    lang::runtime::{
        compile_conundrum::compile_elements,
        parse_conundrum_string::parse_conundrum_string,
        state::{
            citation_list::CitationList,
            parse_state::{ConundrumModifier, ParseState},
        },
    },
    output::parsing_result::{self, mdx_parsing_result::MdxParsingResult},
    parsers::markdown::heading_sluggger::Slugger,
};
use winnow::Stateful;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, uniffi::Record, Clone)]
pub struct ParseMdxOptions {
    pub note_id: Option<String>,
    pub content: String,
    pub modifiers: Vec<ConundrumModifier>,
}

impl ParseMdxOptions {
    pub fn new(note_id: Option<String>, content: String, modifiers: Vec<ConundrumModifier>) -> Self {
        ParseMdxOptions { note_id,
                          content,
                          modifiers }
    }
}

pub async fn run_conundrum(opts: ParseMdxOptions) -> MdxParsingResult {
    // let mut result = MdxParsingResult::from_initial_mdx_content(&opts.content);
    // result.note_id = opts.note_id.clone();
    let state = RefCell::new(ParseState { data: MdxParsingResult::from_initial_mdx_content(&opts.content),
                                          bib: CitationList::default(),
                                          modifiers: opts.modifiers.clone(),
                                          slugger: Slugger::default() });

    let mut stateful_input = Stateful { input: opts.content.as_str(),
                                        state };

    match parse_conundrum_string(&mut stateful_input) {
        Ok((elements, input_data)) => {
            let mut x = input_data.state.borrow_mut();
            let compiled = compile_elements(&elements, &mut x);
            x.data.content = compiled;
            x.data.clone()
        }
        Err(err) => {
            println!("Conundrum Error: {:#?}", err);
            let mut error_state = MdxParsingResult::get_fail_result(err);
            error_state.content = opts.content.clone();
            error_state
        }
    }
}
