use fluster_core_utilities::core_types::syntax::parser_ids::ParserId;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use winnow::{ModalResult, Parser, ascii::alphanumeric1, combinator::delimited, token::literal};

use crate::{
    lang::runtime::traits::mdx_component_result::MdxComponentResult,
    output::parsing_result::mdx_parsing_result::MdxParsingResult, parsers::parser_trait::ConundrumParser,
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

        if res.front_matter
              .as_ref()
              .is_some_and(|fm| fm.ignored_parsers.iter().any(|x| x == &ParserId::Citations.to_string()))
        {
            return self.full_match.clone();
        }

        format!("<FlusterCitation citationKey=\"{}\" />", self.key,)
    }
}

impl ConundrumParser<ParsedCitation> for ParsedCitation {
    fn parse_input_string(input: &mut &str) -> ModalResult<ParsedCitation> {
        let start = *input;

        let key: &str = delimited(literal("[[cite:"), alphanumeric1, literal("]]")).parse_next(input)?;

        let consumed_len = start.len() - input.len();
        let full_match = &start[..consumed_len];

        Ok(ParsedCitation { key: key.to_string(),
                            full_match: full_match.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '['
    }
}
