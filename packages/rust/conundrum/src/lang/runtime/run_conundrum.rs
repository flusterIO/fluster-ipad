use std::{cell::RefCell, sync::Arc};

use askama::Template;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    lang::runtime::{
        compile_conundrum::compile_elements,
        parse_conundrum_string::parse_conundrum_string,
        queries::get_title::get_title_group,
        state::{
            citation_list::CitationList,
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumResult},
            dom_data::DomData,
            parse_state::{ConundrumCompileTarget, ConundrumModifier, ParseState},
            ui_params::UIParams,
        },
    },
    output::{
        general::component_constants::component_names::EmbeddableComponentName,
        html::standalone::standalone_template::StandaloneTemplate,
        parsing_result::mdx_parsing_result::MdxParsingResult,
    },
    parsers::markdown::heading_sluggger::Slugger,
};
use winnow::{Stateful, error::ErrMode};

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
    // let mut result = MdxParsingResult::from_initial_mdx_content(&opts.content);
    // result.note_id = opts.note_id.clone();
    let state = Arc::new(RefCell::new(ParseState { data: MdxParsingResult::from_initial_mdx_content(&opts.content),
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
                                                   trusted: opts.trusted }));
    let b = opts.content.clone();

    let mut stateful_input = Stateful { input: b.as_str(),
                                        state };

    let (elements, input_data) = parse_conundrum_string(&mut stateful_input).map_err(ConundrumErrorVariant::from)?;

    let mut x = input_data.state.borrow_mut();
    let compiled = compile_elements(&elements, &mut x)?;
    if opts.modifiers.contains(&ConundrumModifier::Standalone) {
        let js_string = x.data.compile_javascript_kinda();
        let templ =
            StandaloneTemplate::new(get_title_group(opts.content, opts.modifiers, opts.target).map(|n| n.title).ok(),
                                    compiled,
                                    js_string,
                                    x.ui_params.clone());
        let rendered_standalone = templ.render() .map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })?;
        x.data.content = rendered_standalone;
    } else {
        x.data.content = compiled;
    }
    Ok(x.data.clone())
}
