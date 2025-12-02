use crate::{
    core::core_types::fluster_error::FlusterResult,
    features::mdx::parser::{
        by_regex::regex_parsers::{
            mdx_parser::{MdxParser, ParserId},
            tag_regex_parser::TagRegexParser,
        },
        parsing_result::mdx_parsing_result::MdxParsingResult,
    },
};

#[derive(uniffi::Object)]
pub struct ParseMdxByRegexOpts {
    /// The text to be parsed
    body: String,
    ignore_parsing: Vec<ParserId>,
}

static REGEX_PARSERS: [&'static dyn MdxParser; 1] = [&TagRegexParser];

pub async fn parse_mdx_string_by_regex(opts: ParseMdxByRegexOpts) -> FlusterResult<String> {
    let mut parsers = REGEX_PARSERS.to_vec();
    if !opts.ignore_parsing.is_empty() {
        parsers = REGEX_PARSERS
            .iter()
            .filter(|_parser| {
                opts.ignore_parsing
                    .iter()
                    .any(|ignore_parser| &_parser.parser_id() == ignore_parser)
            })
            .cloned()
            .collect::<Vec<&dyn MdxParser>>();
    }

    let mut result = MdxParsingResult::from_initial_mdx_content(&opts.body);

    for parser in parsers {
        parser.parse_async(&mut result).await;
    }

    Ok("".to_string())
}
