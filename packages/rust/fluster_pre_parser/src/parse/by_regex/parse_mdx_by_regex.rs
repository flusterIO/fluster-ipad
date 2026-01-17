use fluster_core_utilities::core_types::fluster_error::FlusterResult;
use serde::{Deserialize, Serialize};

use crate::{
    parse::by_regex::regex_parsers::{
        citation_regex_parser::CitationRegexParser,
        dictionary_entry_regex_parser::DictionaryEntryRegexParser, mdx_parser::MdxParser,
        note_link_regex_parser::NoteLinkRegexParser, tag_regex_parser::TagRegexParser,
    },
    parsing_result::mdx_parsing_result::MdxParsingResult,
};

static REGEX_PARSERS: [&'static dyn MdxParser; 4] = [
    &TagRegexParser,
    &CitationRegexParser,
    &DictionaryEntryRegexParser,
    &NoteLinkRegexParser,
];

#[derive(Serialize, Deserialize, uniffi::Record)]
pub struct SwiftDataCitationSummary {
    pub citation_key: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, uniffi::Record)]
pub struct ParseMdxOptions {
    pub note_id: Option<String>,
    pub content: String,
    pub citations: Vec<SwiftDataCitationSummary>,
}

pub async fn parse_mdx_string_to_mdx_result(opts: &ParseMdxOptions) -> MdxParsingResult {
    let mut parsers = REGEX_PARSERS.to_vec();
    let mut result = MdxParsingResult::from_initial_mdx_content(&opts.content);
    result.note_id = opts.note_id.clone();
    dbg!(result.clone());
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
        parser.parse_async(opts, &mut result).await;
    }

    result
}

/// ignore_parsing maps to the ParserId enum. This method will eventually be deprecated and replaced by an lsp based approach but this will be a faster way to get up and running.
/// based approach but will work for now.
#[uniffi::export(async_runtime = "tokio")]
pub async fn parse_mdx_string_by_regex(opts: ParseMdxOptions) -> FlusterResult<Vec<u8>> {
    let result = parse_mdx_string_to_mdx_result(&opts).await;

    let data = result.serialize_to_flatbuffer();

    Ok(data)
}

#[cfg(test)]
mod tests {

    use fluster_core_utilities::{
        code_gen::flat_buffer::v1_flat_buffer_schema_generated::mdx_serialization::root_as_mdx_parsing_result_buffer,
        test_utilities::get_test_mdx_content::{
            get_model_note_content, get_welcome_to_fluster_content,
        },
    };

    use super::*;

    #[tokio::test]
    async fn parses_mdx_note_model_by_regex_successfully() {
        let test_content = get_model_note_content();
        let res = parse_mdx_string_by_regex(ParseMdxOptions {
            content: test_content,
            note_id: None,
            citations: Vec::new(),
        })
        .await;
        assert!(
            &res.is_ok(),
            "Parses mdx content without throwing an error."
        );
        let binding = &res.unwrap();
        let result = root_as_mdx_parsing_result_buffer(binding)
            .expect("Deserializes buffer to results without error.");
        assert!(
            !result
                .citations()
                .expect("Result has citations key.")
                .is_empty(),
            "Finds citations in note."
        );
        // assert!(
        //     !result
        //         .dictionary_entries()
        //         .expect("Result has dictionary entries.")
        //         .is_empty(),
        //     "Finds dictionary entries in note."
        // );
        assert!(
            result
                .parsed_content()
                .to_string()
                .contains("<FlusterCitation"),
            "Replaces citation in file."
        );
        // assert!(
        //     result
        //         .parsed_content()
        //         .to_string()
        //         .contains("<DictionaryEntry"),
        //     "Replaces dictionary entry in file."
        // );
    }

    #[tokio::test]
    async fn parses_mdx_by_regex_successfully() {
        let test_content = get_welcome_to_fluster_content();
        let res = parse_mdx_string_by_regex(ParseMdxOptions {
            content: test_content,
            note_id: None,
            citations: Vec::new(),
        })
        .await;
        assert!(
            &res.is_ok(),
            "Parses mdx content without throwing an error."
        );
        let binding = res.unwrap();
        let result = root_as_mdx_parsing_result_buffer(&binding)
            .expect("Deseralizes returned data without throwing an error.");
        assert!(
            result.front_matter().is_some(),
            "Has front matter when front matter is present"
        );
        let fm = result
            .front_matter()
            .expect("Has front matter when front matter is present");

        assert!(
            result
                .parsed_content()
                .to_string()
                .contains("<AutoInsertedTag"),
            "Replaces tag in file."
        );

        assert!(
            fm.user_defined_id()
                .is_some_and(|x| x == "welcomeToFluster"),
            "Note parses user defined id as expected"
        )
        // assert_eq!(result, 4);
    }
}
