use flatbuffers::FlatBufferBuilder;
use fluster_core_utilities::{
    code_gen::flat_buffer::v1_flat_buffer_schema_generated::{
        dictionary::{DictionaryEntryResultBuffer, DictionaryEntryResultBufferArgs},
        mdx_serialization::{
            FrontMatterResultBufferBuilder, ParsedMdxDataTypescriptSafeBuilder, TagResultBuffer, TagResultBufferArgs,
        },
    },
    core_types::fluster_error::FlusterError,
};
use gray_matter::{Matter, engine::YAML};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::output::parsing_result::{
    ai_serialization_request::AiSerializationRequestPhase1,
    dictionary_result::DictionaryEntryResult,
    front_matter::{FrontMatterParser, FrontMatterResult},
    note_outgoing_link_result::NoteOutgoingLinkResult,
    tag_result::TagResult,
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
    /// Always set to false initially, but can be set to true by certain parsers
    /// to avoid further parsing.
    pub ignore_all_parsers: bool,
    pub ai_secondary_parse_requests: Vec<AiSerializationRequestPhase1>,
}

impl Default for MdxParsingResult {
    fn default() -> Self {
        Self { note_id: None,
               content: "".to_string(),
               tags: Vec::new(),
               front_matter: Default::default(),
               ordered_citation_keys: Vec::new(),
               dictionary_entries: Vec::new(),
               outgoing_links: Vec::new(),
               ignore_all_parsers: false,
               ai_secondary_parse_requests: Vec::new() }
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
                             FlusterError::FrontMatterError
                         })
                         .ok();
        MdxParsingResult { note_id: None,
                           content: match &data {
                               Some(d) => d.content.clone(),
                               None => "".to_string(),
                           },
                           tags: Vec::new(),
                           outgoing_links: Vec::new(),
                           ordered_citation_keys: Vec::new(),
                           dictionary_entries: Vec::new(),
                           front_matter: match data {
                               Some(front_matter_data) => {
                                   front_matter_data.data.map(FrontMatterResult::from_gray_matter)
                               }
                               None => None,
                           },
                           ignore_all_parsers: false,
                           ai_secondary_parse_requests: Vec::new() }
    }
}
