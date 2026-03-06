use async_trait::async_trait;
use fluster_core_utilities::core_types::{
    component_constants::{
        component_name_id_map::COMPONENT_NAME_ID_MAP, component_names::EmbeddableComponentName,
        documentation_component_name::DocumentationComponentName,
    },
    documentation_constants::in_content_documentation_id::{
        InContentDocumentationFormat, InContentDocumentationId,
    },
    syntax::parser_ids::ParserId,
};
use strum::IntoEnumIterator;

use crate::{
    embedded::{
        embedded_component_docs::EmbeddedComponentDocs,
        embedded_in_content_docs::EmbeddedInContentDocs,
    },
    parse::by_regex::{parse_mdx_by_regex::ParseMdxOptions, regex_parsers::mdx_parser::MdxParser},
    parsing_result::mdx_parsing_result::MdxParsingResult,
};

pub struct EmbeddedInContentDocsParser;

impl EmbeddedInContentDocsParser {
    async fn parse_internal_documentation(
        &self,
        lines: &mut Vec<String>,
        doc_id: &InContentDocumentationId,
        result: &mut MdxParsingResult,
    ) {
        let cloned_lines = lines.clone();
        for (i, s) in cloned_lines.iter().enumerate() {
            if s == &format!("{}??", doc_id) {
                // Long docs
                result.ignore_all_parsers = true;
                let body_as_string = EmbeddedInContentDocs::get_incontent_docs_by_id(
                    doc_id,
                    &InContentDocumentationFormat::Full,
                );
                lines[i] = format!(
                    "<{} inContentId=\"{}\" format=\"{}\">\n{}\n</{}>",
                    DocumentationComponentName::InContentDocumentationContainer,
                    doc_id,
                    InContentDocumentationFormat::Full,
                    body_as_string,
                    DocumentationComponentName::InContentDocumentationContainer,
                );
            } else if s == &format!("{}?", doc_id) {
                // Short docs
                result.ignore_all_parsers = true;
                let body_as_string = EmbeddedInContentDocs::get_incontent_docs_by_id(
                    doc_id,
                    &InContentDocumentationFormat::Short,
                );
                lines[i] = format!(
                    "<{} inContentId=\"{}\" format=\"{}\">\n{}\n</{}>",
                    DocumentationComponentName::InContentDocumentationContainer,
                    doc_id,
                    InContentDocumentationFormat::Short,
                    body_as_string,
                    DocumentationComponentName::InContentDocumentationContainer,
                );
            }
        }
    }
    async fn parse_component_name(
        &self,
        lines: &mut Vec<String>,
        name: &EmbeddableComponentName,
        result: &mut MdxParsingResult,
    ) {
        if let Some(component_id) = COMPONENT_NAME_ID_MAP.get(name) {
            let cloned_lines = lines.clone();
            for (i, s) in cloned_lines.iter().enumerate() {
                if s == &format!("{}??", name) {
                    result.ignore_all_parsers = true;
                    let body_as_string = EmbeddedComponentDocs::get_incontent_docs_by_id(
                        component_id,
                        &InContentDocumentationFormat::Full,
                    );
                    lines[i] = format!(
                        "<{} componentId=\"{}\" format=\"{}\">\n{}\n</{}>",
                        DocumentationComponentName::InContentDocumentationContainer,
                        component_id,
                        InContentDocumentationFormat::Full,
                        body_as_string,
                        DocumentationComponentName::InContentDocumentationContainer,
                    );
                    // Long docs inserted here
                } else if s == &format!("{}?", name) {
                    result.ignore_all_parsers = true;
                    let body_as_string = EmbeddedComponentDocs::get_incontent_docs_by_id(
                        component_id,
                        &InContentDocumentationFormat::Short,
                    );

                    lines[i] = format!(
                        "<{} componentId=\"{}\" format=\"{}\">\n{}\n</{}>",
                        DocumentationComponentName::InContentDocumentationContainer,
                        component_id,
                        InContentDocumentationFormat::Short,
                        body_as_string,
                        DocumentationComponentName::InContentDocumentationContainer,
                    );
                    // Short docs inserted here
                }
            }
        } else {
            eprintln!(
                "Failed to find a component name id map entry for the {} component",
                name
            );
        }
    }
}

#[async_trait]
impl MdxParser for EmbeddedInContentDocsParser {
    fn parser_id(&self) -> ParserId {
        ParserId::Documentation
    }
    async fn parse_async(&self, _: &ParseMdxOptions, result: &mut MdxParsingResult) {
        let mut new_content = result.content.lines().map(String::from).collect();

        for component_name in EmbeddableComponentName::iter() {
            self.parse_component_name(&mut new_content, &component_name, result)
                .await;
        }
        for internal_documentation in InContentDocumentationId::iter() {
            self.parse_internal_documentation(&mut new_content, &internal_documentation, result)
                .await;
        }
        let joined_body = new_content.join("\n");
        result.content = joined_body;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // TODO: Move this to snapshot testing. Actually dedicate 20 minutes to figuring it out...

    #[tokio::test]
    async fn parses_documentation_properly() {
        let opts = ParseMdxOptions {
            citations: Vec::new(),
            note_id: None,
            content: r#"# My note

Card??
            "#
            .to_string(),
        };
        let mut initial_result = MdxParsingResult::from_initial_mdx_content(&opts.content.clone());

        EmbeddedInContentDocsParser {}
            .parse_async(&opts, &mut initial_result)
            .await;

        insta::assert_snapshot!(initial_result.content);
    }
}
