use askama::Template;
use tw_merge::*;

use crate::lang::lib::ui::shared_props::sizable::SizablePropsGroup;

#[derive(Template)]
#[template(ext = "html", path = "card.html")]
pub struct CardHtmlTemplate {
    pub title: String,
    pub desc: Option<String>,
    pub body: String,
    pub sizable: SizablePropsGroup,
    /// True if user sets `centerContent` OR `centerBody` to true for
    /// compatibility with the sizable type.
    pub center_body: bool,
}
