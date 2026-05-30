use askama::Template;
use tw_merge::*;

use crate::{
    lang::lib::ui::shared_props::sizable::SizablePropsGroup,
    parsers::conundrum::logic::string::conundrum_string::ConundrumString,
};

/// ## Template (HTML)
///
/// ```askama
/// <div class="{{tw_merge!("max-w-[90%] w-full h-fit my-6", self.sizable.as_class(crate::lang::lib::ui::shared_props::sizable::SizablePropsOutputTarget::General))}}">
/// <div class="text-lg font-serif italic{% if self.center_content() %} text-center{% endif %}">
/// {{content | safe}}
/// </div>
/// {% if let Some(source) = self.source %}
/// <div class="text-sm text-mute-foreground">
/// {{source | safe}}
/// </div>
/// {% endif %}
/// </div>
/// ```
#[derive(Template)]
#[template(in_doc = true, ext = "html")]
pub struct QuoteHtmlTemplate {
    pub content: String,
    pub source: Option<String>,
    pub source_id: Option<ConundrumString>,
    pub sizable: SizablePropsGroup,
}

impl QuoteHtmlTemplate {
    pub fn center_content(&self) -> bool {
        self.sizable.center_content.is_some_and(|b| b.0)
    }
}
