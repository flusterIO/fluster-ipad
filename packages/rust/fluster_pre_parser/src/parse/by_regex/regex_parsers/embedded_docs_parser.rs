use async_trait::async_trait;
use fluster_core_utilities::core_types::{
    component_constants::{
        component_name_id_map::COMPONENT_NAME_ID_MAP, component_names::EmbeddableComponentName,
    },
    in_content_documentation_id::{InContentDocumentationFormat, InContentDocumentationId},
};
use regex::Regex;
use strum::IntoEnumIterator;

use crate::{
    embedded::{
        embedded_component_docs::EmbeddedComponentDocs,
        embedded_in_content_docs::EmbeddedInContentDocs,
    },
    parse::by_regex::{
        parse_mdx_by_regex::ParseMdxOptions,
        regex_parsers::mdx_parser::{MdxParser, ParserId},
    },
    parsing_result::mdx_parsing_result::MdxParsingResult,
};

pub struct EmbeddedInContentDocsParser;

impl EmbeddedInContentDocsParser {
    async fn parse_internal_documentation(
        &self,
        content: &str,
        doc_id: &InContentDocumentationId,
    ) -> String {
        let r = Regex::new(&format!("^{}(\\?{{1,2}})", doc_id)).unwrap();
        let mut new_content = String::from(content);
        for result in r.captures_iter(content) {
            if let Some(help_depth) = result.get(1) {
                let complete_match = result.get(0).unwrap().as_str();
                let is_long_docs = help_depth.as_str().len() >= 2;
                let docs_format = match is_long_docs {
                    true => InContentDocumentationFormat::Full,
                    false => InContentDocumentationFormat::Short,
                };
                let body_as_string =
                    EmbeddedInContentDocs::get_incontent_docs_by_id(&doc_id, &docs_format);
                new_content = new_content.replace(
                        complete_match,
                        &format!(
                            "<InContentDocumentationContainer inContentId=\"{}\" format=\"{}\">\n{}\n</InContentDocumentationContainer>",
                            doc_id, docs_format, body_as_string
                        ),
                    );
            }
        }
        new_content
    }
    async fn parse_component_name(&self, content: &str, name: &EmbeddableComponentName) -> String {
        let r = Regex::new(&format!("^{}(\\?{{1,2}})", name)).unwrap();
        let mut new_content = String::from(content);
        for result in r.captures_iter(content) {
            if let Some(help_depth) = result.get(1) {
                let complete_match = result.get(0).unwrap().as_str();
                let is_long_docs = help_depth.as_str().len() >= 2;
                let docs_format = match is_long_docs {
                    true => InContentDocumentationFormat::Full,
                    false => InContentDocumentationFormat::Short,
                };
                if let Some(component_id) = COMPONENT_NAME_ID_MAP.get(name) {
                    let body_as_string =
                        EmbeddedComponentDocs::get_incontent_docs_by_id(component_id, &docs_format);
                    new_content = new_content.replace(
                        complete_match,
                        &format!(
                            "<InContentDocumentationContainer componentId=\"{}\" format=\"{}\">\n{}\n</InContentDocumentationContainer>",
                            component_id, docs_format, body_as_string
                        ),
                    );
                }
            }
        }
        new_content
    }
}

#[async_trait]
impl MdxParser for EmbeddedInContentDocsParser {
    fn parser_id(&self) -> ParserId {
        ParserId::Documentation
    }
    async fn parse_async(&self, _: &ParseMdxOptions, result: &mut MdxParsingResult) {
        let mut new_content = String::from(&result.content);

        for component_name in EmbeddableComponentName::iter() {
            new_content = self
                .parse_component_name(&new_content, &component_name)
                .await;
        }
        for internal_documentation in InContentDocumentationId::iter() {
            new_content = self
                .parse_internal_documentation(&new_content, &internal_documentation)
                .await;
        }
        result.content = new_content;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn parses_note_outgoing_links_properly() {
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

        let should_equal = r#"# My note

<InContentDocumentationContainer
            "#;

        assert!(
            initial_result.content.starts_with(should_equal),
            "Parses InContentDocumentation."
        );
    }
}
