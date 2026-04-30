use askama::Template;
use serde::Serialize;
use winnow::{
    Parser,
    ascii::dec_int,
    combinator::{delimited, preceded},
    error::ErrMode,
    stream::Stream,
};

use crate::{
    lang::runtime::{
        state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
        },
        traits::{
            conundrum_input::{ArcState, ConundrumInput},
            html_js_component_result::HtmlJsComponentResult,
        },
    },
    parsers::{
        conundrum::logic::number::{conundrum_int::ConundrumInt, conundrum_number::ConundrumNumber},
        javascript::javascript_number::javascript_int,
        parser_components::white_space_delimited::white_space_delimited,
        parser_trait::ConundrumParser,
    },
};

/// The `[^1]` syntax that goes in the markdown content, _not_ the footnote that
/// goes in the footer.
///
/// ## Template (HTML)
/// ```askama
/// <sup class="text-sm text-inherit opacity-70 hover:opacity-100 transition-opacity duration-300">{{self.idx.0}}</sup>
/// ```
#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone, Template)]
#[template(ext = "html", in_doc = true)]
pub struct FootnoteAnchor {
    /// The user provided idx.
    pub idx: ConundrumInt,
}

impl HtmlJsComponentResult for FootnoteAnchor {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl ConundrumParser<FootnoteAnchor> for FootnoteAnchor {
    fn parse_input_string(
        input: &mut crate::lang::runtime::traits::conundrum_input::ConundrumInput)
        -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<FootnoteAnchor> {
        let start = input.input.checkpoint();

        '['.void().parse_next(input).inspect_err(|_| {
                                         input.input.reset(&start);
                                     })?;

        '^'.void().parse_next(input).inspect_err(|_| {
                                         input.input.reset(&start);
                                     })?;

        let idx = ConundrumInt::parse_input_string.parse_next(input).inspect_err(|_| {
                                                                         input.input.reset(&start);
                                                                     })?;

        ']'.parse_next(input).inspect_err(|_| {
                                  input.input.reset(&start);
                              })?;

        let mut mutable_state = input.state.write_arc();
        let anchor_id = mutable_state.dom.new_id();
        mutable_state.footnotes.append_footnote_anchor(idx.clone(), anchor_id);
        drop(mutable_state);

        Ok(FootnoteAnchor { idx })
    }

    fn matches_first_char(char: char) -> bool {
        char == '['
    }
}
