use std::sync::Arc;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    lang::runtime::{
        state::conundrum_error_variant::ConundrumModalResult,
        traits::{conundrum_input::ArcState, html_js_component_result::HtmlJsComponentResult},
    },
    output::html::dom::dom_id::DOMId,
    parsers::parsers_shared::timestamp::Timestamp,
};

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum MarkdownLinkTarget {
    /// Any generic url that is not handled internally by the conundrum
    /// framework.
    Url(String),
    /// This currently only supports linking to id's on the same page.
    DomId(DOMId),
    NoteId(String),
    AudioTimestamp(Timestamp),
    VideoTimestamp(Timestamp),
}

impl HtmlJsComponentResult for MarkdownLinkTarget {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        match self {
            Self::Url(u) => Ok(u.clone()),
            Self::NoteId(n) => Ok(n.clone()),
            Self::VideoTimestamp(v) => v.to_html_js_component(Arc::clone(&res)),
            Self::AudioTimestamp(a) => a.to_html_js_component(Arc::clone(&res)),
            Self::DomId(d) => {
                if d.0.starts_with("#") {
                    Ok(d.0.clone())
                } else {
                    Ok(format!("#{}", d.0))
                }
            }
        }
    }
}

impl PartialEq<&str> for MarkdownLinkTarget {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Self::Url(u) => u.as_str() == *other,
            Self::DomId(i) => i.0.as_str() == *other,
            Self::NoteId(i) => i.as_str() == *other,
            Self::AudioTimestamp(a) => a.to_string().as_str() == *other,
            Self::VideoTimestamp(v) => v.to_string().as_str() == *other,
        }
    }
}
