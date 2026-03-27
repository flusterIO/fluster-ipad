use fluster_core_utilities::core_types::{
    component_constants::auto_inserted_component_name::AutoInsertedComponentName, syntax::parser_ids::ParserId,
};
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
    parsers::parser_trait::ConundrumParser,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct ParsedTag {
    /// The tag's body
    pub body: String,
    /// The full match of the content that was originally in the note.
    pub full_match: String,
}

impl PlainTextComponentResult for ParsedTag {
    fn to_plain_text(&self, _: &mut ParseState) -> String {
        self.body.clone()
    }
}

impl FlusterComponentResult for ParsedTag {
    fn to_fluster_component(&self, res: &mut ParseState) -> String {
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MdxComponentResult for ParsedTag {
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

        if res.data
              .front_matter
              .as_ref()
              .is_some_and(|fm| fm.ignored_parsers.iter().any(|x| x == &ParserId::Tags.to_string()))
        {
            return self.full_match.clone();
        }

        format!("<{}>{}</{}>",
                AutoInsertedComponentName::AutoInsertedTag,
                self.body,
                AutoInsertedComponentName::AutoInsertedTag)
    }
}

impl ConundrumParser<ParsedTag> for ParsedTag {
    fn parse_input_string(input: &mut ConundrumInput) -> ModalResult<ParsedTag> {
        let (body, full_match) =
            delimited(literal("[[#"), take_until(1.., "]]"), literal("]]")).with_taken().parse_next(input)?;

        Ok(ParsedTag { body: body.to_string(),
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
        let test_content = "[[#myTag]]";

        let mut test_data = wrap_test_conundrum_content(test_content);

        let res = ParsedTag::parse_input_string(&mut test_data).expect("Parses outgoing link successfully.");

        assert!(test_content.is_empty(), "Citation returns the proper left over text.");

        assert!(res.body == "myTag", "Tag finds the proper body.");
    }
}
