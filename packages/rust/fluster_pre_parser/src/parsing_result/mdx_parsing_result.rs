use flatbuffers::FlatBufferBuilder;
use fluster_core_utilities::{
    code_gen::flat_buffer::v1_flat_buffer_schema_generated::{
        dictionary::{DictionaryEntryResultBuffer, DictionaryEntryResultBufferArgs},
        mdx_serialization::{
            CitationResultBuffer, CitationResultBufferArgs, FrontMatterResultBufferBuilder,
            ParsedMdxDataTypescriptSafeBuilder, TagResultBuffer, TagResultBufferArgs,
        },
    },
    core_types::fluster_error::FlusterError,
};
use gray_matter::{Matter, engine::YAML};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::{
    parse::by_regex::regex_parsers::{
        dictionary_entry_regex_parser::DictionaryEntryResult,
        note_link_regex_parser::NoteOutgoingLinkResult,
    },
    parsing_result::{
        ai_serialization_request::AiSerializationRequestPhase1,
        citation_result::CitationResult,
        front_matter::{FrontMatterParser, FrontMatterResult},
        tag_result::TagResult,
    },
};

#[wasm_bindgen]
#[derive(uniffi::Record, Debug, Serialize, Deserialize, Clone)]
pub struct MdxParsingResult {
    pub(crate) note_id: Option<String>,
    pub(crate) content: String,
    pub(crate) tags: Vec<TagResult>,
    pub(crate) front_matter: Option<FrontMatterResult>,
    /// bibliography_string is a string representing the concatenated bibtex entries of all
    /// valid bibentries within the note, without duplicates and in the proper order.
    pub(crate) citations: Vec<CitationResult>,
    pub(crate) dictionary_entries: Vec<DictionaryEntryResult>,
    pub(crate) outgoing_links: Vec<NoteOutgoingLinkResult>,
    /// Always set to false initially, but can be set to true by certain parsers to avoid further
    /// parsing.
    pub(crate) ignore_all_parsers: bool,
    pub(crate) ai_secondary_parse_requests: Vec<AiSerializationRequestPhase1>,
}

#[wasm_bindgen]
impl MdxParsingResult {
    #[wasm_bindgen(constructor)]
    pub fn new(
        note_id: Option<String>,
        content: String,
        tags: Vec<TagResult>,
        front_matter: Option<FrontMatterResult>,
        citations: Vec<CitationResult>,
        dictionary_entries: Vec<DictionaryEntryResult>,
        outgoing_links: Vec<NoteOutgoingLinkResult>,
        ignore_all_parsers: bool,
        ai_secondary_parse_requests: Vec<AiSerializationRequestPhase1>,
    ) -> Self {
        MdxParsingResult {
            note_id,
            content,
            tags,
            front_matter,
            citations,
            dictionary_entries,
            outgoing_links,
            ignore_all_parsers,
            ai_secondary_parse_requests,
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn get_content_rust(&self) -> String {
        self.content.clone()
    }

    pub fn set_front_matter(&mut self, front_matter: Option<FrontMatterResult>) {
        self.front_matter = front_matter;
    }

    pub fn get_front_matter_rust(&self) -> Option<FrontMatterResult> {
        self.front_matter.clone()
    }
    /// GETTER: Converts the entire Rust struct into a native JS Object.
    /// This is O(n) but allows TypeScript to access nested fields like
    /// `citations` and `tags` instantly without further WASM calls.
    #[wasm_bindgen(getter)]
    pub fn json(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap_or(JsValue::NULL)
    }

    /// SETTER: Updates the Rust struct from a JavaScript object.
    /// This is vital for your Swift-to-JS-to-Rust pipeline.
    #[wasm_bindgen(setter)]
    pub fn set_from_json(&mut self, val: JsValue) {
        if let Ok(new_data) = serde_wasm_bindgen::from_value::<MdxParsingResult>(val) {
            *self = new_data;
        }
    }

    #[wasm_bindgen(js_name = getCitations)]
    pub fn get_citations(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.citations).unwrap_or(JsValue::UNDEFINED)
    }

    /// This will not return a parsed item, but rather an empty template that can be passed to a
    /// series of MdxParsers to apply the necessary properties and changes.
    pub fn from_initial_mdx_content(content: &str) -> MdxParsingResult {
        let matter = Matter::<YAML>::new();
        let data = matter
            .parse(content)
            .map_err(|e| {
                println!("Front Matter Error: {}", e);
                FlusterError::FrontMatterError
            })
            .ok();
        MdxParsingResult {
            note_id: None,
            content: match &data {
                Some(d) => d.content.clone(),
                None => "".to_string(),
            },
            citations: Vec::new(),
            tags: Vec::new(),
            outgoing_links: Vec::new(),
            dictionary_entries: Vec::new(),
            front_matter: match data {
                Some(front_matter_data) => front_matter_data
                    .data
                    .map(FrontMatterResult::from_gray_matter),
                None => None,
            },
            ignore_all_parsers: false,
            ai_secondary_parse_requests: Vec::new(),
        }
    }

    // Deprecated? Fucking hell I hope so...
    pub fn serialize_to_flatbuffer(&self) -> Vec<u8> {
        let mut builder: FlatBufferBuilder = FlatBufferBuilder::new();
        let content = builder.create_string(&self.content);

        // --- Serialize Tags ---
        let mut tags_offsets = Vec::new();
        for tag in &self.tags {
            let body = builder.create_string(&tag.body);
            tags_offsets.push(TagResultBuffer::create(
                &mut builder,
                &TagResultBufferArgs { body: Some(body) },
            ));
        }
        let tags_vector = builder.create_vector(&tags_offsets);

        // --- Serialize Citations ---
        let mut citations_offsets = Vec::new();
        for citation in &self.citations {
            let key = builder.create_string(&citation.citation_key);
            citations_offsets.push(CitationResultBuffer::create(
                &mut builder,
                &CitationResultBufferArgs {
                    citation_key: Some(key),
                    idx: citation.idx,
                },
            ));
        }

        let citations_vector = builder.create_vector(&citations_offsets);

        // --- Serializing Dictionary Entries ---

        let mut dictionary_entry_offets = Vec::new();

        for dict in &self.dictionary_entries {
            let label = builder.create_string(&dict.label);
            let body = builder.create_string(&dict.body);
            let note_id = self.note_id.clone().map(|y| builder.create_string(&y));
            dictionary_entry_offets.push(DictionaryEntryResultBuffer::create(
                &mut builder,
                &DictionaryEntryResultBufferArgs {
                    label: Some(label),
                    body: Some(body),
                    note_id,
                },
            ))
        }

        let dictionary_entries = builder.create_vector(&dictionary_entry_offets);

        let front_matter_offset = self.front_matter.as_ref().map(|fm| {
            let ignored_parsers_offsets = fm
                .ignored_parsers
                .iter()
                .map(|s| builder.create_string(s))
                .collect::<Vec<_>>();
            let ignored_parsers_vector = builder.create_vector(&ignored_parsers_offsets);

            let title = fm.title.as_ref().map(|s| builder.create_string(s));
            let user_defined_id = fm
                .user_defined_id
                .as_ref()
                .map(|s| builder.create_string(s));
            let mut fmb = FrontMatterResultBufferBuilder::new(&mut builder);
            fmb.add_ignore_parsers(ignored_parsers_vector);
            if let Some(title) = title {
                fmb.add_title(title);
            }
            if let Some(user_def_id) = user_defined_id {
                fmb.add_user_defined_id(user_def_id);
            }
            fmb.finish()
        });

        // let ai_parsing_requests_builder = SecondaryAiParseRequestBuilder::new(&mut builder);

        // ai_parsing_requests_builder.add_type_(SecondaryAiParsingRequestType::CreateStudyGuide);

        // let ai_secondary_parse_requests = builder.create_vector(&unimplemented_ai_parsing_requests);

        // let requires_ai_parse = builder.create

        let mut mdx_builder = ParsedMdxDataTypescriptSafeBuilder::new(&mut builder);
        mdx_builder.add_parsed_content(content);
        mdx_builder.add_tags(tags_vector);
        mdx_builder.add_dictionary_entries(dictionary_entries);

        if let Some(front_matter_offset) = front_matter_offset {
            mdx_builder.add_front_matter(front_matter_offset);
        }
        mdx_builder.add_citations(citations_vector);

        // mdx_builder.add_requires_secondary_ai_parse(ai_secondary_parse_requests);

        let mdx_parsing_result_buff = mdx_builder.finish();
        builder.finish(mdx_parsing_result_buff, None);
        builder.finished_data().to_vec()
    }
}
