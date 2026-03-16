use fluster_core_utilities::core_types::syntax::parser_ids::ParserId;
use nom::{
    IResult, Parser,
    bytes::{complete::tag, take_until},
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
pub struct ParsedOutgoingNoteLink {
    /// The user-defined id of the note which is being linked to.
    pub note_id: String,
    /// The text content of the link. `[The stuff here](note:MyNote)`
    pub content: String,
    /// The full content of the input string that represents this outgoing note link.
    pub full_match: String,
}

impl MdxComponentResult for ParsedOutgoingNoteLink {
    fn to_mdx_component(&self, res: &mut MdxParsingResult) -> String {
        if res.ignore_all_parsers {
            return self.full_match.clone();
        }

        if let Some(fm) = &res.front_matter {
            if fm
                .ignored_parsers
                .iter()
                .any(|x| x == &ParserId::NoteLink.to_string())
            {
                return self.full_match.clone();
            }
        }
        format!(
            "<NoteLink id=\"{}\">{}</NoteLink>",
            self.note_id, self.content
        )
    }
}

pub fn parse_outgoing_note_link(input: &str) -> IResult<&str, ParsedOutgoingNoteLink> {
    let start_point = input;
    let (i, display_text) = delimited(char('['), take_until("]"), char(']')).parse(input)?;

    let (i, note_id) = delimited(tag("(note:"), take_until(")"), char(')')).parse(i)?;

    let consumed_len = start_point.len() - i.len();
    let full_match = &start_point[..consumed_len];

    Ok((
        i,
        ParsedOutgoingNoteLink {
            note_id: note_id.to_string(),
            content: display_text.to_string(),
            full_match: full_match.to_string(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsed_link() {
        let test_content = "[This is my link](note:myNote)";

        let (left_over, res) =
            parse_outgoing_note_link(test_content).expect("Parses outgoing link successfully.");

        assert!(
            left_over == "",
            "Citation returns the proper left over text."
        );

        assert!(
            res.content == "This is my link",
            "Citation finds the proper content."
        );
        assert!(
            res.note_id == "myNote",
            "Citation finds the proper note id."
        );
    }
}
