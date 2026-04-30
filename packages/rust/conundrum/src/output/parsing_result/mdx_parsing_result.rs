use dashmap::DashMap;
use gray_matter::{Matter, engine::YAML};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typeshare::typeshare;

use crate::{
    lang::runtime::state::{conundrum_error::ConundrumError, conundrum_error_variant::ConundrumErrorVariant},
    output::{
        html::{dom::dom_id::DOMId, glue::component_glue_manager::AnyComponentKey},
        parsing_result::{
            ai_serialization_request::AiSerializationRequestPhase1,
            dictionary_result::DictionaryEntryResult,
            front_matter::{FrontMatterParser, FrontMatterResult},
            note_outgoing_link_result::NoteOutgoingLinkResult,
            tag_result::TagResult,
        },
    },
    parsers::{
        conundrum::logic::number::conundrum_int::ConundrumInt,
        markdown::{
            heading::heading_model::MarkdownHeadingStringifiedResult,
            markdown_extensions::footnote::footnote_result::RenderedFootnoteResult,
        },
    },
};

#[typeshare]
#[derive(uniffi::Record, Debug, Serialize, Deserialize, Clone)]
pub struct MdxParsingResult {
    pub note_id: Option<String>,
    pub content: String,
    pub tags: Vec<TagResult>,
    pub front_matter: Option<FrontMatterResult>,
    pub ordered_citation_keys: Vec<String>,
    pub dictionary_entries: Vec<DictionaryEntryResult>,
    pub outgoing_links: Vec<NoteOutgoingLinkResult>,
    pub toc: Vec<MarkdownHeadingStringifiedResult>,
    /// Always set to false initially, but can be set to true by certain parsers
    /// to avoid further parsing.
    pub ignore_all_parsers: bool,
    pub ai_secondary_parse_requests: Vec<AiSerializationRequestPhase1>,
    /// The map of the user provided equation id (to the `EqRef` component) and
    /// the index that the equation appears.
    pub eq_ref_map: HashMap<String, u32>,
    pub warnings: Vec<ConundrumError>,
    /// A map of type `Map<The anchor index of the footnote, FootnoteResult>`.
    pub footnotes: HashMap<ConundrumInt, Option<RenderedFootnoteResult>>,
    included_components: Vec<AnyComponentKey>,
}

impl Default for MdxParsingResult {
    fn default() -> Self {
        Self { note_id: None,
               content: "".to_string(),
               tags: Vec::new(),
               front_matter: Default::default(),
               ordered_citation_keys: Vec::new(),
               dictionary_entries: Vec::new(),
               toc: Vec::new(),
               footnotes: HashMap::new(),
               outgoing_links: Vec::new(),
               ignore_all_parsers: false,
               eq_ref_map: HashMap::new(),
               warnings: Vec::new(),
               included_components: Vec::new(),
               ai_secondary_parse_requests: Vec::new() }
    }
}

impl MdxParsingResult {
    pub fn contains_tag(&self, tag_body: &str) -> bool {
        self.tags.iter().any(|x| x.body == tag_body)
    }

    pub fn contains_outgoing_link(&self, link_note_id: &str) -> bool {
        self.outgoing_links.iter().any(|x| x.link_to_note_id == link_note_id)
    }

    pub fn get_included_components(&self) -> Vec<AnyComponentKey> {
        self.included_components.clone()
    }

    pub fn append_embeddable_component(&mut self, name: &AnyComponentKey) {
        if !self.included_components.iter().any(|x| x == name) {
            self.included_components.push(name.clone());
        }
    }
}

impl MdxParsingResult {
    /// This will not return a parsed item, but rather an empty template that
    /// can be passed to a series of MdxParsers to apply the necessary
    /// properties and changes.
    pub fn from_initial_mdx_content(content: &str) -> MdxParsingResult {
        let matter = Matter::<YAML>::new();
        let data = matter.parse(content)
                         .map_err(|e| {
                             println!("Front Matter Error: {}", e);
                             ConundrumErrorVariant::FrontMatterError
                         })
                         .ok();
        MdxParsingResult { note_id: None,
                           content: match &data {
                               Some(d) => d.content.clone(),
                               None => "".to_string(),
                           },
                           tags: Vec::new(),
                           outgoing_links: Vec::new(),
                           toc: Vec::new(),
                           ordered_citation_keys: Vec::new(),
                           dictionary_entries: Vec::new(),
                           footnotes: HashMap::new(),
                           front_matter: match data {
                               Some(front_matter_data) => {
                                   front_matter_data.data.map(FrontMatterResult::from_gray_matter)
                               }
                               None => None,
                           },
                           ignore_all_parsers: false,
                           eq_ref_map: HashMap::new(),
                           warnings: Vec::new(),
                           included_components: Vec::new(),
                           ai_secondary_parse_requests: Vec::new() }
    }
}
