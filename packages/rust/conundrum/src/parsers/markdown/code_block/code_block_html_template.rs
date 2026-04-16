use askama::Template;

use crate::output::html::dom::dom_id::DOMId;

/// ## Template (HTML)
///
/// ```askama
/// <div class="cdrm-codeblock h-fit w-full bg-fd-card text-fd-card-foreground border rounded
/// relative">
///   {% if let Some(title) = title %}
///   <div
///   class="px-2 py-0 text-fd-card-foreground/80"
///   >
///   {{title}}
///   </div>
///   {% endif %}
///    <div class="w-full relative" id="{{id}}">
///    <div
///      data-cdrm-copy-for="{{id}}"
///      role="button"
///      class="auto-codeblock-icon absolute top-2 right-2 transition-opacity duration-300"
///      onclick="copyCodeblockCode"
///    >
///    {{crate::output::html::icons::embedded_web_icons::EmbeddedIcon::Copy}}
///    </div>
///    <div class="[&>pre]:p-2 [&>code]:rounded-bl [&>code]:rounded-br">
///    {{code}}
///    </div>
///    </div>
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct CodeBlockHTMLTemplate {
    pub id: DOMId,
    pub code: String,
    pub title: Option<String>,
}

impl CodeBlockHTMLTemplate {
    pub fn new(code: String, title: Option<String>, id: DOMId) -> Self {
        CodeBlockHTMLTemplate { id,
                                code,
                                title }
    }
}
