use askama::Template;
use serde::Serialize;
use winnow::{Parser, error::ErrMode, stream::Stream};

use crate::{
    lang::runtime::{
        state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            footnote_manager::FootnoteData,
        },
        traits::{
            conundrum_input::{ArcState, ConundrumInput},
            html_js_component_result::HtmlJsComponentResult,
        },
    },
    output::html::dom::dom_id::DOMId,
    parsers::{conundrum::logic::number::conundrum_int::ConundrumInt, parser_trait::ConundrumParser},
};

/// The `[^1]` syntax that goes in the markdown content, _not_ the footnote that
/// goes in the footer.
///
/// ## Template (HTML)
/// ```askama
/// <sup id="{{id}}" class="cdrm-footnote-anchor text-sm text-inherit opacity-70 hover:opacity-100 transition-opacity duration-300 cursor-pointer">{{self.doc_idx}}</sup>
/// ```
#[typeshare::typeshare]
#[derive(Debug, Serialize, serde::Deserialize, Clone, Template)]
#[template(ext = "html", in_doc = true)]
pub struct FootnoteAnchor {
    /// The user provided idx.
    pub idx: ConundrumInt,
    pub doc_idx: ConundrumInt,
    pub id: DOMId,
}

impl HtmlJsComponentResult for FootnoteAnchor {
    fn to_html_js_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        self.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl ConundrumParser<FootnoteAnchor> for FootnoteAnchor {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<FootnoteAnchor> {
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
        let cloned_state = mutable_state.clone();
        let found_item = cloned_state.footnotes.0.get(&idx);
        let anchor_id = match &found_item {
            Some(s) => s.get_id(),
            None => {
                let anchor_id = mutable_state.dom.new_id();
                mutable_state.footnotes.append_footnote_anchor(idx, anchor_id.clone());
                anchor_id
            }
        };
        let doc_idx = match &found_item {
            Some(s) => match s {
                FootnoteData::Completed(c) => Ok(c.idx),
                FootnoteData::Rendered(r) => Ok(r.idx),
                FootnoteData::Assigned(a) => Ok(a.idx + 1),
            },
            None => Ok(ConundrumInt(mutable_state.footnotes.0.len() as i64)),
        }?;
        drop(mutable_state);

        Ok(FootnoteAnchor { idx,
                            doc_idx,
                            id: anchor_id })
    }

    fn matches_first_char(char: char) -> bool {
        char == '['
    }
}
