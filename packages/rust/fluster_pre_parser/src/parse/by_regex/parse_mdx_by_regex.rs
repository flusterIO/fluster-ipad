use fluster_core_utilities::core_types::fluster_error::FlusterResult;
use serde::{Deserialize, Serialize};

use crate::{
    parse::by_regex::regex_parsers::{mdx_parser::MdxParser, tag_regex_parser::TagRegexParser},
    parsing_result::mdx_parsing_result::MdxParsingResult,
};

static REGEX_PARSERS: [&'static dyn MdxParser; 1] = [&TagRegexParser];

#[derive(Serialize, Deserialize, uniffi::Record)]
pub struct ParseMdxOptions {
    content: String,
    ignore_parsing: Vec<u8>,
}

// impl Lift {
//
// }

/// ignore_parsing maps to the ParserId enum. This method will eventually be deprecated and replaced by an lsp based approach but this will be a faster way to get up and running.
/// based approach but will work for now.
#[uniffi::export(async_runtime = "tokio")]
pub async fn parse_mdx_string_by_regex(opts: ParseMdxOptions) -> FlusterResult<MdxParsingResult> {
    let mut parsers = REGEX_PARSERS.to_vec();
    if !opts.ignore_parsing.is_empty() {
        parsers = REGEX_PARSERS
            .iter()
            .filter(|_parser| {
                opts.ignore_parsing
                    .iter()
                    .any(|ignore_parser| _parser.parser_id() as u8 == *ignore_parser)
            })
            .cloned()
            .collect::<Vec<&dyn MdxParser>>();
    }

    let mut result = MdxParsingResult::from_initial_mdx_content(&opts.content);

    for parser in parsers {
        parser.parse_async(&mut result).await;
    }

    Ok(result)
}
