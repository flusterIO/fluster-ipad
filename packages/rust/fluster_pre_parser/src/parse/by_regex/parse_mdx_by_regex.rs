use fluster_core_utilities::core_types::fluster_error::FlusterResult;
use serde::{Deserialize, Serialize};

use crate::{
    parse::by_regex::regex_parsers::{
        citation_regex_parser::CitationRegexParser, mdx_parser::MdxParser,
        tag_regex_parser::TagRegexParser,
    },
    parsing_result::mdx_parsing_result::MdxParsingResult,
};

static REGEX_PARSERS: [&'static dyn MdxParser; 2] = [&TagRegexParser, &CitationRegexParser];

#[derive(Serialize, Deserialize, uniffi::Record)]
pub struct SwiftDataCitationSummary {
    pub citation_key: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, uniffi::Record)]
pub struct ParseMdxOptions {
    pub content: String,
    pub citations: Vec<SwiftDataCitationSummary>,
}

// impl Lift {
//
// }

/// ignore_parsing maps to the ParserId enum. This method will eventually be deprecated and replaced by an lsp based approach but this will be a faster way to get up and running.
/// based approach but will work for now.
#[uniffi::export(async_runtime = "tokio")]
pub async fn parse_mdx_string_by_regex(opts: ParseMdxOptions) -> FlusterResult<MdxParsingResult> {
    let mut parsers = REGEX_PARSERS.to_vec();
    let mut result = MdxParsingResult::from_initial_mdx_content(&opts.content);
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
        parser.parse_async(&opts, &mut result).await;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {

    use fluster_core_utilities::test_utilities::get_test_mdx_content::get_welcome_to_fluster_content;

    use super::*;

    #[tokio::test]
    async fn parses_mdx_by_regex_successfully() {
        let test_content = get_welcome_to_fluster_content();
        let res = parse_mdx_string_by_regex(ParseMdxOptions {
            content: test_content,
            citations: Vec::new(),
        })
        .await;
        assert!(
            &res.is_ok(),
            "Parses mdx content without throwing an error."
        );
        assert!(
            res.as_ref().unwrap().front_matter.is_some(),
            "Has front matter when front matter is present"
        );
        let fm = res.as_ref().unwrap().front_matter.as_ref().unwrap();
        assert!(
            fm.ignored_parsers.iter().any(|x| x == "tags"),
            "Has 'tags' parser in 'ignore_parsers' when present in that field."
        );

        assert!(
            !res.unwrap().content.contains("AutoInsertedTag"),
            "No tags were inserted when 'tags' parser was ignored."
        );
        // assert_eq!(result, 4);
    }
}
