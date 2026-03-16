use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char},
    sequence::delimited,
};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    lang::runtime::traits::mdx_component_result::MdxComponentResult,
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct ParsedCitation {
    pub key: String,
    pub full_match: String,
}

impl MdxComponentResult for ParsedCitation {
    fn to_mdx_component(&self, _: &mut MdxParsingResult) -> String {
        format!("<FlusterCitation citationKey='{}' />", self.key,)
    }
}

pub fn parse_citation(input: &str) -> IResult<&str, ParsedCitation> {
    let start_point = input;

    let (remaining, key) = delimited(tag("[cite:"), alphanumeric1, char(']')).parse(input)?;

    let consumed_len = start_point.len() - remaining.len();
    let full_match = &start_point[..consumed_len];

    Ok((
        remaining,
        ParsedCitation {
            key: key.to_string(),
            full_match: full_match.to_string(),
        },
    ))
}
