use parking_lot::RwLock;
use std::sync::Arc;

use winnow::Stateful;

use crate::{
    lang::runtime::state::{citation_list::CitationList, dom_data::DomData, parse_state::ParseState},
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
    parsers::markdown::heading_sluggger::Slugger,
};

pub type ArcState = Arc<RwLock<ParseState>>;
pub type ConundrumInput<'a> = Stateful<&'a str, ArcState>;

pub fn get_conundrum_input(val: &str, state: ParseState) -> ConundrumInput {
    ConundrumInput { input: val,
                     state: Arc::new(RwLock::new(ParseState { data:
                                                                  MdxParsingResult::from_initial_mdx_content(val),
                                                              bib: CitationList::default(),
                                                              modifiers: state.modifiers.clone(),
                                                              eq_count: 0,
                                                              last_heading_depth: 0,
                                                              last_heading_tab_depth: 0,
                                                              valid_footnote_indices: Vec::new(),
                                                              ui_params: state.ui_params.clone(),
                                                              dom: DomData::default(),
                                                              compile_target: state.compile_target.clone(),
                                                              slugger: Slugger::default(),
                                                              trusted: state.trusted.clone(),
                                                              ..Default::default() })) }
}

pub fn duplicate_conundrum_input(new_value: &str, state: ArcState) -> ConundrumInput {
    ConundrumInput { input: new_value,
                     state }
}
