use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, sync::Arc};
use typeshare::typeshare;

use crate::{
    lang::runtime::state::{
        citation_list::CitationList,
        conundrum_error_variant::{ConundrumErrorVariant, ConundrumResult},
        dom_data::DomData,
        parse_state::{ConundrumCompileTarget, ConundrumModifier, ParseState},
        ui_params::UIParams,
    },
    output::{
        general::component_constants::component_names::EmbeddableComponentName,
        parsing_result::mdx_parsing_result::MdxParsingResult,
    },
    parsers::{document::ConundrumDocument, markdown::heading_sluggger::Slugger},
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
    pub ui_params: UIParams,
    pub target: ConundrumCompileTarget,
    pub trusted: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for ParseConundrumOptions {
    fn default() -> Self {
        Self { note_id: None,
               content: String::from(""),
               modifiers: vec![],
               hide_components: vec![],
               ui_params: UIParams::default(),
               target: ConundrumCompileTarget::Html,
               trusted: false }
    }
}

impl ParseConundrumOptions {
    pub fn new(note_id: Option<String>,
               content: String,
               modifiers: Vec<ConundrumModifier>,
               hide_components: Vec<EmbeddableComponentName>,
               ui_params: UIParams,
               target: ConundrumCompileTarget,
               trusted: bool)
               -> Self {
        ParseConundrumOptions { note_id,
                                content,
                                modifiers,
                                hide_components,
                                ui_params,
                                target,
                                trusted }
    }
}

pub async fn run_conundrum(opts: ParseConundrumOptions) -> ConundrumResult<MdxParsingResult> {
    let state = Arc::new(RwLock::new(ParseState { data: MdxParsingResult::from_initial_mdx_content(&opts.content),
                                                  bib: CitationList::default(),
                                                  modifiers: opts.modifiers.clone(),
                                                  eq_count: 0,
                                                  last_heading_depth: 0,
                                                  last_heading_tab_depth: 0,
                                                  valid_footnote_indices: Vec::new(),
                                                  ui_params: opts.ui_params.clone(),
                                                  dom: DomData { id_count: 0 },
                                                  compile_target: ConundrumCompileTarget::Html,
                                                  slugger: Slugger::default(),
                                                  trusted: opts.trusted,
                                                  ..Default::default() }));
    let b = opts.content.clone();

    let mut stateful_input = Stateful { input: b.as_str(),
                                        state };

    let is_standalone = opts.modifiers.contains(&ConundrumModifier::Standalone);
    let doc = ConundrumDocument::parse_input(&mut stateful_input).map_err(ConundrumErrorVariant::from)?;

    println!("Doc: {:#?}", doc);

    // let mut state = stateful_input.state.borrow_mut();
    // let rendered_string = match is_standalone {
    //     true => doc.render_standalone(Arc::clone(&stateful_input.state))?,
    //     false => doc.render_app_embedded(Arc::clone(&stateful_input.state))?,
    // };

    println!("Rendered: {:#?}", is_standalone);
    // let mut state = stateful_input.state.borrow_mut();

    // state.data.content = rendered_string;

    // Ok(state.data.clone())
    Ok(MdxParsingResult::default())
}
