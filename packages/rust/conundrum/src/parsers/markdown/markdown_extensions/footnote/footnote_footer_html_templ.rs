use crate::parsers::{
    conundrum::logic::number::conundrum_int::ConundrumInt,
    markdown::markdown_extensions::footnote::{
        footnote_anchor::FootnoteAnchor, footnote_result::RenderedFootnoteResult,
    },
};
use askama::Template;

/// ## Template (HTML)
///
/// ```askama
/// {% if self.footnotes.len() > 0 %}
/// <div class="cdrm-footnotes w-full max-w-[1080px] h-fit">
/// <h4>Footnotes</h4>
/// <div class="w-full grid grid-cols-[auto_1fr] gap-x-4">
/// {% for footnote in self.footnotes %}
/// {{footnote.render()? | safe}}
/// {% endfor %}
/// </div>
/// </div>
/// {% endif %}
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct FootnoteSectionTemplate {
    pub footnotes: Vec<RenderedFootnoteResult>,
}
