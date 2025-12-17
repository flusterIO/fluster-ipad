use flatbuffers::{FlatBufferBuilder, WIPOffset};
use fluster_core_utilities::{
    code_gen::flat_buffer::v1_flat_buffer_schema_generated::mdx_serialization::{
        CitationResultBuffer, CitationResultBufferArgs, FrontMatterResultBufferBuilder,
        MdxParsingResultBuffer, MdxParsingResultBufferBuilder, TagResultBuffer,
        TagResultBufferArgs,
    },
    core_types::fluster_error::FlusterError,
};
use gray_matter::{Matter, engine::YAML};
use serde::{Deserialize, Serialize};

use crate::parsing_result::{
    citation_result::CitationResult, front_matter::FrontMatterResult, tag_result::TagResult,
};

#[derive(uniffi::Record, Serialize, Deserialize, Clone)]
pub struct MdxParsingResult {
    pub content: String,
    pub tags: Vec<TagResult>,
    pub front_matter: Option<FrontMatterResult>,
    /// bibliography_string is a string representing the concatenated bibtex entries of all
    /// valid bibentries within the note, without duplicates and in the proper order.
    pub citations: Vec<CitationResult>,
}

impl MdxParsingResult {
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
            content: match &data {
                Some(d) => d.content.clone(),
                None => "".to_string(),
            },
            citations: Vec::new(),
            tags: Vec::new(),
            front_matter: match data {
                Some(front_matter_data) => front_matter_data
                    .data
                    .map(FrontMatterResult::from_gray_matter),
                None => None,
            },
        }
    }

    pub fn serialize_to_flatbuffer<'builder>(&self) -> Vec<u8> {
        let mut builder: FlatBufferBuilder<'builder> = FlatBufferBuilder::new();
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
            let body = builder.create_string(&citation.body);
            citations_offsets.push(CitationResultBuffer::create(
                &mut builder,
                &CitationResultBufferArgs {
                    citation_key: Some(key),
                    body: Some(body),
                },
            ));
        }

        let citations_vector = builder.create_vector(&citations_offsets);

        let front_matter_offset = self.front_matter.as_ref().map(|fm| {
            let ignored_parsers_offsets = fm
                .ignored_parsers
                .iter()
                .map(|s| builder.create_string(s))
                .collect::<Vec<_>>();
            let ignored_parsers_vector = builder.create_vector(&ignored_parsers_offsets);

            let title = fm.title.as_ref().map(|s| builder.create_string(s));
            println!("fm.user_defined_id: {:#?}", fm.user_defined_id);
            let user_defined_id = fm
                .user_defined_id
                .as_ref()
                .map(|s| builder.create_string(s));
            let mut fmb = FrontMatterResultBufferBuilder::new(&mut builder);
            fmb.add_ignore_parsers(ignored_parsers_vector);
            if let Some(title) = title {
                fmb.add_title(title);
            }
            println!("Is some: {:#?}", user_defined_id);
            if let Some(user_def_id) = user_defined_id {
                println!("User defined id here: {:#?}", user_def_id);
                fmb.add_user_defined_id(user_def_id);
            }
            fmb.finish()
        });

        let mut mdx_builder = MdxParsingResultBufferBuilder::new(&mut builder);
        mdx_builder.add_parsed_content(content);
        mdx_builder.add_tags(tags_vector);
        if let Some(front_matter_offset) = front_matter_offset {
            mdx_builder.add_front_matter(front_matter_offset);
        }
        mdx_builder.add_citations(citations_vector);
        let mdx_parsing_result_buff = mdx_builder.finish();
        builder.finish(mdx_parsing_result_buff, None);
        builder.finished_data().to_vec()
    }
}
