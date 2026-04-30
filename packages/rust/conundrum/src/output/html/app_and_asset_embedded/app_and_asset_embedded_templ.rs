use crate::{
    lang::runtime::state::{
        conundrum_error::ConundrumError,
        conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
    },
    parsers::markdown::markdown_extensions::footnote::{
        footnote_footer_html_templ::FootnoteSectionTemplate, footnote_result::RenderedFootnoteResult,
    },
};
use askama::Template;
use winnow::error::{ErrMode, Result};

/// ## Template (HTML)
///
/// ```askama
/// {% if let Some(style) = self.css %}<style>
/// {{style | safe}}
/// </style>{% endif %}
/// <div class="cdrm-body-container w-full h-fit">
/// {{content | safe}}
/// {{self.footnotes.render()? | safe}}
/// </div> {% if let Some(js) = self.js %}
/// <script>
/// {{js | safe}}
/// console.info("App glue code loaded...")
/// </script> {% endif %}
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct AppAndAssetEmbedded {
    pub content: String,
    pub footnotes: FootnoteSectionTemplate,
    pub js: Option<String>,
    pub css: Option<String>,
}
