use askama::Template;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use winnow::{
    Parser,
    combinator::delimited,
    error::ErrMode,
    token::{literal, take_until},
};

use crate::{
    lang::runtime::{
        state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            parse_state::{ConundrumCompileTarget, ParseState},
        },
        traits::{
            conundrum_input::ConundrumInput, fluster_component_result::ConundrumComponentResult,
            html_js_component_result::HtmlJsComponentResult, mdx_component_result::MdxComponentResult,
            plain_text_component_result::PlainTextComponentResult,
        },
    },
    output::general::component_constants::parser_ids::ParserId,
    parsers::parser_trait::ConundrumParser,
};

/// ## Template (HTML)
/// ```askama
/// <sup data-cdrm-cit-key="{{key}}">{{idx + 1}}</sup>
/// ```
#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug, Template)]
#[template(ext = "html", in_doc = true)]
pub struct ParsedCitation {
    pub key: String,
    pub full_match: String,
    pub idx: u32,
}

impl HtmlJsComponentResult for ParsedCitation {
    fn to_html_js_component(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
        self.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl PlainTextComponentResult for ParsedCitation {
    fn to_plain_text(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(self.key.clone())
    }
}

impl ConundrumComponentResult for ParsedCitation {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if res.compile_target == ConundrumCompileTarget::PlainText {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MdxComponentResult for ParsedCitation {
    fn to_mdx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if res.data.ignore_all_parsers {
            return Ok(self.full_match.clone());
        }

        if res.data
              .front_matter
              .as_ref()
              .is_some_and(|fm| fm.ignored_parsers.iter().any(|x| x == &ParserId::Citations.to_string()))
        {
            return Ok(self.full_match.clone());
        }

        Ok(format!("<FlusterCitation citationKey=\"{}\" idx={{{}}} />", self.key, self.idx))
    }
}

impl ConundrumParser<ParsedCitation> for ParsedCitation {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<ParsedCitation> {
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
