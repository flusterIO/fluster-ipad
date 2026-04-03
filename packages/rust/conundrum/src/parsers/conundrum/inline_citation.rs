use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use winnow::{
    ModalResult, Parser,
    combinator::delimited,
    token::{literal, take_until},
};

use crate::{
    lang::runtime::{
        state::{
            conundrum_error_variant::ConundrumResult,
            parse_state::{ConundrumModifier, ParseState},
        },
        traits::{
            conundrum_input::ConundrumInput, fluster_component_result::ConundrumComponentResult,
            mdx_component_result::MdxComponentResult, plain_text_component_result::PlainTextComponentResult,
        },
    },
    output::general::component_constants::parser_ids::ParserId,
    parsers::parser_trait::ConundrumParser,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct ParsedCitation {
    pub key: String,
    pub full_match: String,
    pub idx: u32,
}

impl PlainTextComponentResult for ParsedCitation {
    fn to_plain_text(&self, _: &mut ParseState) -> String {
        self.key.clone()
    }
}

impl ConundrumComponentResult for ParsedCitation {
    fn to_conundrum_component(&self, res: &mut ParseState) -> String {
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MdxComponentResult for ParsedCitation {
    fn to_mdx_component(&self, res: &mut ParseState) -> String {
        if res.data.ignore_all_parsers {
            return self.full_match.clone();
        }

        if res.data
              .front_matter
              .as_ref()
              .is_some_and(|fm| fm.ignored_parsers.iter().any(|x| x == &ParserId::Citations.to_string()))
        {
            return self.full_match.clone();
        }

        format!("<FlusterCitation citationKey=\"{}\" idx={{{}}} />", self.key, self.idx)
    }
}

impl ConundrumParser<ParsedCitation> for ParsedCitation {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumResult<ParsedCitation> {
        let (key, full_match) =
            delimited(literal("[[cite:"), take_until(1.., "]]"), literal("]]")).with_taken().parse_next(input)?;

        let mut state = input.state.borrow_mut();

        let idx = state.bib.get_item_index_and_append(key);

        Ok(ParsedCitation { key: key.to_string(),
                            full_match: full_match.to_string(),
                            idx: idx as u32 })
    }

    fn matches_first_char(char: char) -> bool {
        char == '['
    }
}
