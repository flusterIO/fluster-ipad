use fluster_core_utilities::core_types::{
    component_constants::{
        component_name_id_map::COMPONENT_NAME_ID_MAP, component_names::EmbeddableComponentName,
        documentation_component_name::DocumentationComponentName,
    },
    documentation_constants::in_content_documentation_id::{
        InContentDocumentationFormat, InContentDocumentationId,
    },
};
use nom::{IResult, Parser, branch::alt, character::complete::line_ending};
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::space0,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use typeshare::typeshare;

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
            } else if let Some(component_name) =
                EmbeddableComponentName::iter().find(|x| x.to_string() == self.keyword)
            {
                if let Some(doc_id) = COMPONENT_NAME_ID_MAP.get(&component_name) {
                    let body_as_string =
                        EmbeddedComponentDocs::get_incontent_docs_by_id(&doc_id, &depth);
                    return format!(
                        "\n<{} inContentId=\"{}\" format=\"{}\">\n{}\n</{}>\n",
                        DocumentationComponentName::InContentDocumentationContainer,
                        doc_id,
                        depth,
                        body_as_string,
                        DocumentationComponentName::InContentDocumentationContainer,
                    );
                }
            }
        }
        self.full_match.clone()
    }
}

pub fn parse_inspection(input: &str) -> IResult<&str, ParsedInspectionRequest> {
    let start_point = input;

    // 1. Match the keyword (alphanumeric)
    let (i, keyword) = take_while1(|c: char| c.is_alphanumeric())(input)?;

    // 2. Match exactly one or two question marks
    let mut alt_tags = alt((tag("??"), tag("?")));
    let (i, marks) = alt_tags.parse(i)?;

    // 3. CRITICAL: Ensure it is the end of the line (no trailing non-whitespace)
    let (i, _) = space0(i)?;
    let (i, _) = alt((line_ending, tag(""))).parse(i)?;

    let level = if marks == "??" { 2 } else { 1 };
    let consumed_len = start_point.len() - i.len();

    Ok((
        i,
        ParsedInspectionRequest {
            keyword: keyword.to_string(),
            level,
            full_match: start_point[..consumed_len].to_string(),
        },
    ))
}
