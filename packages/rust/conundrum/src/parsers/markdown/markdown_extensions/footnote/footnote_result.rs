use crate::{
    lang::{
        lib::ui::ui_types::children::Children,
        runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ArcState},
    },
    output::html::dom::dom_id::DOMId,
    parsers::conundrum::logic::number::conundrum_int::ConundrumInt,
};
use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct PartialFootnote {
    /// The id of the anchor element associated with this footnote.
    pub anchor_id: DOMId,
    /// The 0 based index of the anchor, the way it appears in the document...
    /// Not necessarily the user provided index that is used as the key of
    /// the footnote results map.
    pub idx: ConundrumInt,
}

#[derive(Debug, Clone)]
pub struct FootnoteResult {
    /// The rendered Conundrum content representing the body of the footnote.
    pub body: Children,
    /// The id of the anchor element associated with this footnote.
    pub anchor_id: DOMId,
    /// The 0 based index of the anchor, the way it appears in the document...
    /// Not necessarily the user provided index that is used as the key of
    /// the footnote results map.
    pub idx: ConundrumInt,
}

/// ## Template (HTML)
///
/// ```askama
/// <a role="button" data-cdrm-for="{{anchor_id}}"  class="cdrm-footnote w-full h-full text-foreground/80 hover:text-foreground transition-colors duration-300 flex flex-col justify-start items-start cursor-pointer  text-sm py-1 not-prose">
/// {{idx | safe}}
/// </a>
/// <div class="cdrm-footnote-body w-full not-prose">
/// {{body | safe}}
/// </div>
/// ```
#[typeshare::typeshare]
#[derive(Debug, uniffi::Record, Serialize, Deserialize, Clone, Template)]
#[template(ext = "html", in_doc = true)]
pub struct RenderedFootnoteResult {
    /// The rendered Conundrum content representing the body of the footnote.
    pub body: String,
    /// The id of the anchor element associated with this footnote.
    pub anchor_id: DOMId,
    /// The 0 based index of the anchor, the way it appears in the document...
    /// Not necessarily the user provided index that is used as the key of
    /// the footnote results map.
    pub idx: ConundrumInt,
}

impl FootnoteResult {
    pub fn to_rendered_footnote(&self, state: ArcState) -> ConundrumModalResult<RenderedFootnoteResult> {
        Ok(RenderedFootnoteResult { body: self.body.render(state)?,
                                    anchor_id: self.anchor_id.clone(),
                                    idx: self.idx + 1 })
    }
}
