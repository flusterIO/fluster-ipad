use fluster_core_utilities::core_types::syntax::parser_ids::ParserId;
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
    fn to_mdx_component(&self, res: &mut MdxParsingResult) -> String {
        if res.ignore_all_parsers {
            return self.full_match.clone();
        }

        if let Some(fm) = &res.front_matter {
            if fm
                .ignored_parsers
                .iter()
                .any(|x| x == &ParserId::Citations.to_string())
            {
                return self.full_match.clone();
            }
        }

        format!("<FlusterCitation citationKey=\"{}\" />", self.key,)
    }
}

pub fn parse_citation(input: &str) -> IResult<&str, ParsedCitation> {
    let start_point = input;

    let (remaining, key) = delimited(tag("[[cite:"), alphanumeric1, tag("]]")).parse(input)?;

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
