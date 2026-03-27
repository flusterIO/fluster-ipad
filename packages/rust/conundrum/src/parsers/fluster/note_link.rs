use fluster_core_utilities::core_types::syntax::parser_ids::ParserId;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use winnow::{
    ModalResult, Parser,
    combinator::delimited,
    token::{literal, take_until},
};

use crate::{
    lang::runtime::{
        state::parse_state::{ConundrumModifier, ParseState},
        traits::{
            conundrum_input::ConundrumInput, fluster_component_result::FlusterComponentResult,
            mdx_component_result::MdxComponentResult, plain_text_component_result::PlainTextComponentResult,
        },
    },
    output::parsing_result::note_outgoing_link_result::NoteOutgoingLinkResult,
    parsers::parser_trait::ConundrumParser,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct ParsedOutgoingNoteLink {
    /// The user-defined id of the note which is being linked to.
    pub note_id: String,
    /// The text content of the link. `[The stuff here](note:MyNote)`
    pub content: String,
    /// The full content of the input string that represents this outgoing note
    /// link.
    pub full_match: String,
}

impl PlainTextComponentResult for ParsedOutgoingNoteLink {
    fn to_plain_text(&self, _: &mut ParseState) -> String {
        self.full_match.clone()
    }
}

impl FlusterComponentResult for ParsedOutgoingNoteLink {
    fn to_fluster_component(&self, res: &mut ParseState) -> String {
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MdxComponentResult for ParsedOutgoingNoteLink {
    fn to_mdx_component(&self, res: &mut ParseState) -> String {
        if res.data.ignore_all_parsers {
            return self.full_match.clone();
        }

        if res.data
              .front_matter
              .as_ref()
              .is_some_and(|fm| fm.ignored_parsers.iter().any(|x| x == &ParserId::NoteLink.to_string()))
        {
            return self.full_match.clone();
        }

        if !res.data.contains_outgoing_link(&self.note_id) {
            res.data.outgoing_links.push(NoteOutgoingLinkResult { link_to_note_id: self.note_id.clone() });
        }

        format!("<NoteLink id=\"{}\">{}</NoteLink>", self.note_id, self.content)
    }
}

impl ConundrumParser<ParsedOutgoingNoteLink> for ParsedOutgoingNoteLink {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ModalResult<ParsedOutgoingNoteLink> {
        let ((display_text, note_id), full_match) = (
            delimited('[', take_until(1.., ']'), ']'),
            delimited(literal("(note:"), take_until(1.., ")"), ')')
        )
        .with_taken()
        .parse_next(input)?;

        Ok(ParsedOutgoingNoteLink { note_id: note_id.to_string(),
                                    content: display_text.to_string(),
                                    full_match: full_match.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '['
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parsed_link() {
        let test_content = "[This is my link](note:myNote)";
        let mut test_data = wrap_test_conundrum_content(test_content);

        let res =
            ParsedOutgoingNoteLink::parse_input_string(&mut test_data).expect("Parses outgoing link successfully.");

        assert!(test_content.is_empty(), "Citation returns the proper left over text.");

        assert!(res.content == "This is my link", "Citation finds the proper content.");
        assert!(res.note_id == "myNote", "Citation finds the proper note id.");
    }
}
