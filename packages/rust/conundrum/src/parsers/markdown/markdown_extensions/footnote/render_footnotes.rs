use std::{collections::HashMap, sync::Arc};

use winnow::error::ErrMode;

use crate::{
    lang::runtime::{
        state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            footnote_manager::FootnoteData,
        },
        traits::conundrum_input::ArcState,
    },
    parsers::{
        conundrum::logic::number::conundrum_int::ConundrumInt,
        markdown::markdown_extensions::footnote::footnote_result::RenderedFootnoteResult,
    },
};

/// **BEWARE**: This mutates the state, applying the rendered footnotes to the
/// returned portion of the state. This must be called once and only once.
pub fn render_footnotes(state: ArcState) -> ConundrumModalResult<Vec<RenderedFootnoteResult>> {
    let mut items: Vec<RenderedFootnoteResult> = Vec::new();
    let read_state = state.read_arc();
    let footnotes = read_state.footnotes.clone();
    drop(read_state);
    let mut footnote_map: HashMap<ConundrumInt, RenderedFootnoteResult> = HashMap::new();
    for (key, value) in footnotes.0.iter() {
        let r = match value {
            FootnoteData::Rendered(r) => Ok(r.clone()),
            FootnoteData::Completed(x) => x.to_rendered_footnote(Arc::clone(&state)),
            FootnoteData::Assigned(x) => {
                Err(ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Forgot something...",
                                                                                                                 "Conundrum found a footnote anchor without an associated footer. For every `[^1]` anchor, there must also be a matching `[^1]: My footnote...` footer."))))
            }
        }?;
        items.push(r.clone());
        footnote_map.insert(*key, r.clone());
    }

    let mut write_state = state.write_arc();
    write_state.data.footnotes = footnote_map;
    drop(write_state);
    items.sort_by(|a, b| a.idx.0.cmp(&b.idx.0));
    Ok(items)
}
