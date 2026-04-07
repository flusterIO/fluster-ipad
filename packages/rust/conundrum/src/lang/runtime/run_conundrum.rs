use std::cell::RefCell;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    lang::runtime::{
        compile_conundrum::compile_elements,
        parse_conundrum_string::parse_conundrum_string,
        state::{
            citation_list::CitationList,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumResult},
            parse_state::{ConundrumModifier, ParseState},
        },
    },
    output::{
        general::component_constants::component_names::EmbeddableComponentName,
        parsing_result::mdx_parsing_result::MdxParsingResult,
    },
    parsers::markdown::heading_sluggger::Slugger,
};
use winnow::Stateful;

/// This is the core 'input' for Conundrum.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, uniffi::Record, Clone)]
pub struct ParseConundrumOptions {
    /// The id of your note. This can be generic, unique to each application,
    /// but so-long as the note has the **concept** of an id the id can be
    /// inserted into certain components to make it easier for the front-end
    /// to collect data without the need for complicated outside state.
    pub note_id: Option<String>,
    /// Your conundrum content, obviously.
    pub content: String,
    /// Kind of the core piece of Conundrum, the ability to write in a superset
    /// of Markdown and to compile to a variety of targets, including just
    /// regular, traditional markdown (the goal is to comply with the Commonmark
    /// spec eventually, we're still missing some things that are being cleaned
    /// up by the runtime environment).
    pub modifiers: Vec<ConundrumModifier>,
    /// Hide specific components that are incompatable with the target output
    /// environment. This is done automatically with the `.ForAiInput` flag
    /// for all AI input components so not to confuse the AI, but there
    /// maybe other use cases as well. Any component added here will render
    /// _nothing_, but it's state will still be applied to the result.
    pub hide_components: Vec<EmbeddableComponentName>,
}

impl ParseConundrumOptions {
    pub fn new(note_id: Option<String>,
               content: String,
               modifiers: Vec<ConundrumModifier>,
               hide_components: Vec<EmbeddableComponentName>)
               -> Self {
        ParseConundrumOptions { note_id,
                                content,
                                modifiers,
                                hide_components }
    }
}

pub async fn run_conundrum(opts: ParseConundrumOptions) -> ConundrumResult<MdxParsingResult> {
    // let mut result = MdxParsingResult::from_initial_mdx_content(&opts.content);
    // result.note_id = opts.note_id.clone();
    let state = RefCell::new(ParseState { data: MdxParsingResult::from_initial_mdx_content(&opts.content),
                                          bib: CitationList::default(),
                                          modifiers: opts.modifiers.clone(),
                                          eq_count: 0,
                                          slugger: Slugger::default() });

    let mut stateful_input = Stateful { input: opts.content.as_str(),
                                        state };

    let (elements, input_data) = parse_conundrum_string(&mut stateful_input).map_err(ConundrumErrorVariant::from)?;

    let mut x = input_data.state.borrow_mut();
    let compiled = compile_elements(&elements, &mut x)?;
    x.data.content = compiled;
    Ok(x.data.clone())
}
