use std::sync::Arc;

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
    parsers::markdown::markdown_extensions::footnote::footnote_result::RenderedFootnoteResult,
};

pub fn render_footnotes(state: ArcState) -> ConundrumModalResult<Vec<RenderedFootnoteResult>> {
    let mut items: Vec<RenderedFootnoteResult> = Vec::new();
    let read_state = state.read_arc();
    let footnotes = read_state.footnotes.clone();
    drop(read_state);
    for (_, value) in footnotes.0.iter() {
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
