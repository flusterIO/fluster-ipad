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
pub struct ParsedCitation {
    pub key: String,
    pub full_match: String,
    pub idx: u32,
}

impl MdxComponentResult for ParsedCitation {
    fn to_mdx_component(&self, res: &mut MdxParsingResult) -> String {
        if res.ignore_all_parsers {
            return self.full_match.clone();
        }

        if res.front_matter
              .as_ref()
              .is_some_and(|fm| fm.ignored_parsers.iter().any(|x| x == &ParserId::Citations.to_string()))
        {
            return self.full_match.clone();
        }

        format!("<FlusterCitation citationKey=\"{}\" idx={{{}}} />", self.key, self.idx)
    }
}

impl ConundrumParser<ParsedCitation> for ParsedCitation {
    fn parse_input_string(input: &mut ConundrumInput) -> ModalResult<ParsedCitation> {
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
