use fluster_core_utilities::core_types::{
    component_constants::{
        component_name_id_map::COMPONENT_NAME_ID_MAP, component_names::EmbeddableComponentName,
        documentation_component_name::DocumentationComponentName,
    },
    documentation_constants::in_content_documentation_id::{
        InContentDocumentationFormat, InContentDocumentationId,
    },
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use typeshare::typeshare;
use winnow::{
    ModalResult, Parser,
    ascii::{line_ending, space0},
    combinator::alt,
    stream::{Offset, Stream},
    token::{literal, take_while},
};

use crate::{
    embedded::{
        embedded_component_docs::EmbeddedComponentDocs,
        embedded_in_content_docs::EmbeddedInContentDocs,
    },
    lang::runtime::traits::mdx_component_result::MdxComponentResult,
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct ParsedInspectionRequest {
    pub keyword: String,
    pub level: u8, // 1 for '?', 2 for '??'
    pub full_match: String,
}

impl MdxComponentResult for ParsedInspectionRequest {
    fn to_mdx_component(&self, res: &mut MdxParsingResult) -> String {
        res.ignore_all_parsers = true;
        if let Some(depth) = match self.level {
            1 => Some(InContentDocumentationFormat::Short),
            2 => Some(InContentDocumentationFormat::Full),
            _ => None,
        } {
            if let Some(doc_id) =
                InContentDocumentationId::iter().find(|x| x.to_string() == self.keyword)
            {
                let body_as_string =
                    EmbeddedInContentDocs::get_incontent_docs_by_id(&doc_id, &depth);
                return format!(
                    "\n<{} inContentId=\"{}\" format=\"{}\">\n{}\n</{}>\n",
                    DocumentationComponentName::InContentDocumentationContainer,
                    doc_id,
                    depth,
                    body_as_string,
                    DocumentationComponentName::InContentDocumentationContainer,
                );
            } else if let Some(comp_id) = EmbeddableComponentName::iter()
                .find(|x| x.to_string() == self.keyword)
                .map(|component_name| {
                    COMPONENT_NAME_ID_MAP
                        .get(&component_name)
                        .expect("All components must have documentation.")
                })
            {
                let body_as_string =
                    EmbeddedComponentDocs::get_incontent_docs_by_id(comp_id, &depth);
                return format!(
                    "\n<{} componentId=\"{}\" format=\"{}\">\n{}\n</{}>\n",
                    DocumentationComponentName::InContentDocumentationContainer,
                    comp_id,
                    depth,
                    body_as_string,
                    DocumentationComponentName::InContentDocumentationContainer,
                );
            }
        }
        self.full_match.clone()
    }
}

pub fn parse_inspection(input: &mut &str) -> ModalResult<ParsedInspectionRequest> {
    let start_point = input.checkpoint();

    // 1. Match the keyword (alphanumeric)
    let keyword = take_while(1.., |c: char| c.is_alphanumeric()).parse_next(input)?;

    // 2. Match exactly one or two question marks
    let mut alt_tags = alt((literal("??"), literal("?")));
    let marks = alt_tags.parse_next(input)?;

    // 3. CRITICAL: Ensure it is the end of the line (no trailing non-whitespace)
    let _ = space0.parse_next(input)?;
    let _ = alt((line_ending, literal(""))).parse_next(input)?;

    let level = if marks == "??" { 2 } else { 1 };

    // Full Match
    let end_point = input.checkpoint();
    let offset = end_point.offset_from(&start_point);
    let full_match = input.peek_slice(offset);

    Ok(ParsedInspectionRequest {
        keyword: keyword.to_string(),
        level,
        full_match: full_match.to_string(),
    })
}
