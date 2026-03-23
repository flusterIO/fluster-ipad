use fluster_core_utilities::core_types::syntax::parser_ids::ParserId;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use winnow::{
    ModalResult, Parser,
    combinator::delimited,
    token::{literal, take_until},
};

use crate::{
    lang::runtime::traits::{conundrum_input::ConundrumInput, mdx_component_result::MdxComponentResult},
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
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

impl MdxComponentResult for ParsedOutgoingNoteLink {
    fn to_mdx_component(&self, res: &mut MdxParsingResult) -> String {
        if res.ignore_all_parsers {
            return self.full_match.clone();
        }

        if res.front_matter
              .as_ref()
              .is_some_and(|fm| fm.ignored_parsers.iter().any(|x| x == &ParserId::NoteLink.to_string()))
        {
            return self.full_match.clone();
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
