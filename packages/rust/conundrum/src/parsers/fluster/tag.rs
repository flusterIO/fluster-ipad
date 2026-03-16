use fluster_core_utilities::core_types::syntax::parser_ids::ParserId;
use nom::{
    IResult, Parser,
    bytes::{complete::tag, take_until},
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
pub struct ParsedTag {
    /// The tag's body
    pub body: String,
    /// The full match of the content that was originally in the note.
    pub full_match: String,
}

impl MdxComponentResult for ParsedTag {
    fn to_mdx_component(&self, res: &mut MdxParsingResult) -> String {
        if res.ignore_all_parsers {
            return self.full_match.clone();
        }

        if let Some(fm) = &res.front_matter {
            if fm
                .ignored_parsers
                .iter()
                .any(|x| x == &ParserId::Tags.to_string())
            {
                return self.full_match.clone();
            }
        }

        format!("<AutoInsertedTag>{}</AutoInsertedTag>", self.body)
    }
}

pub fn parse_tags(input: &str) -> IResult<&str, ParsedTag> {
    let start_point = input;
    let (i, body) = delimited(tag("[[#"), take_until("]]"), tag("]]")).parse(input)?;

    let consumed_len = start_point.len() - i.len();
    let full_match = &start_point[..consumed_len];
    Ok((
        i,
        ParsedTag {
            body: body.to_string(),
            full_match: full_match.to_string(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsed_link() {
        let test_content = "[[#myTag]]";

        let (left_over, res) =
            parse_tags(test_content).expect("Parses outgoing link successfully.");

        assert!(
            left_over.is_empty(),
            "Citation returns the proper left over text."
        );

        assert!(res.body == "myTag", "Tag finds the proper body.");
    }
}
