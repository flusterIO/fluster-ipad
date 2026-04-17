use askama::Template;
use devicons::{FileIcon, Theme};

use crate::{
    output::html::dom::dom_id::DOMId, parsers::markdown::code_block::supported_languages::SupportedCodeBlockSyntax,
};

/// ## Template (HTML)
///
/// ```askama
/// <div class="cdrm-codeblock h-fit w-full bg-fd-card text-fd-card-foreground border rounded relative">   {% if let Some(title) = title %}
///   <div class="my-6 px-2 py-0 text-fd-card-foreground/80 text-sm w-full grid grid-cols-[auto_1fr] gap-x-2">
///   <div style="color:{{file_icon.color}};">
///   {{file_icon.icon}}
///   </div>
///   <div>
///   {{title}}
///   </div>
///   </div> {% endif %}
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
///    {{code | safe}}
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
    pub file_icon: FileIcon,
}

impl CodeBlockHTMLTemplate {
    pub fn new(code: String,
               title: Option<String>,
               id: DOMId,
               language: &SupportedCodeBlockSyntax,
               dark_mode: &bool)
               -> Self {
        let file_icon = match &title {
            Some(n) => devicons::icon_for_file(n,
                                               &Some(match dark_mode {
                                                         true => Theme::Dark,
                                                         false => Theme::Light,
                                                     })),
            None => language.devicon(dark_mode),
        };

        CodeBlockHTMLTemplate { id,
                                code,
                                title,
                                file_icon }
    }
}
