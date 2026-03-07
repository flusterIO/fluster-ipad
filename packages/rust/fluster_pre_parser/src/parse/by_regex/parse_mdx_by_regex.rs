use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    parse::by_regex::regex_parsers::{
        ai_trigger_parser::AiTriggerParser, citation_regex_parser::CitationRegexParser,
        dictionary_entry_regex_parser::DictionaryEntryRegexParser,
        embedded_docs_parser::EmbeddedInContentDocsParser, mdx_parser::MdxParser,
        note_link_regex_parser::NoteLinkRegexParser, tag_regex_parser::TagRegexParser,
    },
    parsing_result::mdx_parsing_result::MdxParsingResult,
};

static REGEX_PARSERS: [&'static dyn MdxParser; 6] = [
    // Keep the EmbeddedInContentDocsParser first to allow the inserted content to then be parsed
    // if needed and to set the ignore_all field if neccessary.
    &EmbeddedInContentDocsParser,
    &TagRegexParser,
    &CitationRegexParser,
    &DictionaryEntryRegexParser,
    &NoteLinkRegexParser,
    &AiTriggerParser,
];

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, uniffi::Record)]
pub struct SwiftDataCitationSummary {
    citation_key: String,
    body: String,
}

#[wasm_bindgen]
impl SwiftDataCitationSummary {
    #[wasm_bindgen(getter)]
    pub fn citation_key(&self) -> *const u8 {
        self.citation_key.as_ptr()
    }
    #[wasm_bindgen(setter)]
    pub fn set_citation_key(&mut self, citation_key: String) {
        self.citation_key = citation_key
    }
}

// impl SwiftDataCitationSummary {
//     #[wasm_bindgen(getter)]
//     pub fn citation_key(&self) -> String {
//         self.citation_key
//     }
// }

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, uniffi::Record)]
pub struct ParseMdxOptions {
    pub(crate) note_id: Option<String>,
    pub(crate) content: String,
    pub(crate) citations: Vec<SwiftDataCitationSummary>,
}

#[wasm_bindgen]
impl ParseMdxOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        note_id: Option<String>,
        citations: Vec<SwiftDataCitationSummary>,
        content: String,
    ) -> Self {
        ParseMdxOptions {
            note_id,
            content,
            citations,
        }
    }

    pub fn get_content_rust(&self) -> String {
        self.content.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn content(&self) -> *const u8 {
        self.content.as_ptr()
    }
    #[wasm_bindgen(setter)]
    pub fn set_content(&mut self, content: String) {
        self.content = content
    }
    #[wasm_bindgen(getter)]
    pub fn citations(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.citations).unwrap_or(JsValue::null())
    }
    #[wasm_bindgen(setter)]
    pub fn set_citations(&mut self, citations: Vec<SwiftDataCitationSummary>) {
        self.citations = citations
    }
}

/// ignore_parsing maps to the ParserId enum. This method will eventually be deprecated and replaced by an lsp based approach but this will be a faster way to get up and running.
/// based approach but will work for now.
pub async fn parse_mdx_string_to_mdx_result(opts: &ParseMdxOptions) -> MdxParsingResult {
    let mut parsers = REGEX_PARSERS.to_vec();
    let mut result = MdxParsingResult::from_initial_mdx_content(&opts.content);
    result.note_id = opts.note_id.clone();
    if let Some(ref fm) = result.front_matter
        && !fm.ignored_parsers.is_empty()
    {
        parsers = REGEX_PARSERS
            .iter()
            .filter(|_parser| {
                !fm.ignored_parsers
                    .iter()
                    .any(|ignore_parser| _parser.parser_id().to_string() == *ignore_parser)
            })
            .cloned()
            .collect::<Vec<&dyn MdxParser>>();
    }

    for parser in parsers {
        if !result.ignore_all_parsers {
            parser.parse_async(opts, &mut result).await;
        }
    }

    result
}

#[cfg(test)]
mod tests {

    use fluster_core_utilities::test_utilities::get_test_mdx_content::{
        get_test_note_content_with_everything, get_welcome_to_fluster_content,
    };

    use super::*;

    #[tokio::test]
    async fn parses_mdx_note_model_by_regex_successfully() {
        let test_content = get_test_note_content_with_everything();
        let res = parse_mdx_string_to_mdx_result(&ParseMdxOptions {
            content: test_content,
            note_id: None,
            citations: Vec::new(),
        })
        .await;
        assert!(
            res.front_matter
                .expect("Finds front matter when front-matter exists.")
                .title
                .expect("Finds front-matter title")
                == "My Notes title",
            "Parses title fron front-matter"
        );

        assert!(
            res.dictionary_entries
                .iter()
                .any(|x| x.label == "My dictionary entry"),
            "Finds dictionary entries."
        );
        assert!(
            res.content.to_string().contains("<DictionaryEntry"),
            "Replaces dictionary entries."
        );
        assert!(
            res.citations
                .iter()
                .any(|x| x.citation_key == "myCitationHere"),
            "Finds citations in note."
        );
        assert!(
            res.content.to_string().contains("<FlusterCitation"),
            "Replaces citation in file."
        );
    }

    #[tokio::test]
    async fn parses_mdx_by_regex_successfully() {
        let test_content = get_welcome_to_fluster_content();
        let res = parse_mdx_string_to_mdx_result(&ParseMdxOptions {
            content: test_content,
            note_id: None,
            citations: Vec::new(),
        })
        .await;

        assert!(
            res.front_matter.is_some(),
            "Has front matter when front matter is present"
        );
        let fm = res
            .front_matter
            .expect("Has front matter when front matter is present");

        assert!(
            res.content.to_string().contains("<AutoInsertedTag"),
            "Replaces tag in file."
        );

        assert!(
            fm.user_defined_id.is_some_and(|x| x == "welcomeToFluster"),
            "Note parses user defined id as expected"
        )
        // assert_eq!(result, 4);
    }
}
