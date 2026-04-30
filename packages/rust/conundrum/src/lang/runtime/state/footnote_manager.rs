use std::{collections::HashMap, error::Error, sync::Arc};
use winnow::error::ErrMode;

use crate::{
    lang::{
        lib::ui::ui_types::children::Children,
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            },
            traits::conundrum_input::ArcState,
        },
    },
    output::html::dom::dom_id::DOMId,
    parsers::{
        conundrum::logic::number::conundrum_int::ConundrumInt,
        markdown::markdown_extensions::footnote::footnote_result::{
            FootnoteResult, PartialFootnote, RenderedFootnoteResult,
        },
    },
};

#[derive(Clone, Debug)]
pub enum FootnoteData {
    /// The state when the footnote anchor is reached but before the footer is
    /// found.
    Assigned(PartialFootnote),
    /// The state once the footer has been found and combined with the data from
    /// the first phast.
    Completed(FootnoteResult),
    /// The rendered footnote, ready to be returned to the client.
    Rendered(RenderedFootnoteResult),
}

#[derive(Debug, Clone)]
pub struct FootnoteManager(HashMap<ConundrumInt, FootnoteData>);

impl FootnoteManager {
    pub fn new(data: HashMap<ConundrumInt, FootnoteData>) -> Self {
        FootnoteManager(data)
    }

    fn get_new_item_index(&self, key: ConundrumInt) -> ConundrumInt {
        if let Some(data) = self.0.get(&key) {
            match data {
                FootnoteData::Assigned(x) => x.idx,
                FootnoteData::Completed(c) => c.idx,
                FootnoteData::Rendered(r) => r.idx,
            }
        } else {
            let n = self.0.len();
            ConundrumInt(n as i64)
        }
    }

    pub fn append_footnote_anchor(&mut self, key: ConundrumInt, anchor_id: DOMId) {
        if self.0.get(&key).is_none() {
            let citation_idx = self.get_new_item_index(key);
            self.0.insert(key,
                          FootnoteData::Assigned(PartialFootnote { anchor_id,
                                                                   idx: citation_idx }));
        }
    }

    /// Will throw a `ConundrumModalError` if the key does not exist.
    ///
    /// **key**: The user defined index of the citation for lookup.
    /// **idx**: The 0 based index based on where it appears
    /// in the document, not necessarily the
    /// user provided index.
    pub fn apply_footnote_footer(&mut self, key: &ConundrumInt, body: Children) -> ConundrumModalResult<()> {
        if let Some((anchor_id, idx)) = self.0.get(key).map(|item| match item {
                                                           FootnoteData::Rendered(r) => (r.anchor_id.clone(), r.idx),
                                                           FootnoteData::Completed(c) => (c.anchor_id.clone(), c.idx),
                                                           FootnoteData::Assigned(c) => (c.anchor_id.clone(), c.idx),
                                                       })
        {
            self.0.insert(key.clone(),
                          FootnoteData::Completed(FootnoteResult { body: body.clone(),
                                                                   idx,
                                                                   anchor_id }));
            Ok(())
        } else {
            Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid footnote",
                                                                                                                   "Conundrum found a footnote footer that does not have a valid footnote anchor."))))
        }
    }

    pub fn get_current_footnote_idx(&self) -> usize {
        self.0.len()
    }

    pub fn to_rendered_footnotes(&self, state: ArcState) -> ConundrumModalResult<Vec<RenderedFootnoteResult>> {
        let mut items: Vec<RenderedFootnoteResult> = Vec::new();
        for (key, value) in self.0.iter() {
            let r = match value {
                FootnoteData::Rendered(r) => Ok(r.clone()),
                FootnoteData::Completed(x) => x.to_rendered_footnote(Arc::clone(&state)),
                FootnoteData::Assigned(x) => {
                    Err(ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Forgot something...",
                                                                                                                     "Conundrum found a footnote anchor without an associated footer. For every `[^1]` anchor, there must also be a matching `[^1]: My footnote...` footer."))))
                }
            }?;
            items.push(r.clone());
        }
        Ok(items)
    }
}
