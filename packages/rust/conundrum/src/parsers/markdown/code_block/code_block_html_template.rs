use askama::Template;
use devicons::{FileIcon, Theme};

use crate::{
    output::html::dom::dom_id::DOMId, parsers::markdown::code_block::supported_languages::SupportedCodeBlockSyntax,
};

/// ## Template (HTML)
///
/// ```askama
/// <div class="cdrm-codeblock @container/codeblock h-fit w-full bg-fd-card text-fd-card-foreground border rounded relative my-6">   {% if let Some(title) = title %}
///   <div class="my-2 px-2 py-0 text-sm w-full grid grid-cols-[auto_1fr] gap-x-2">
///   <div class="w-fit" style="color:{{file_icon.color}};">
///   {{file_icon.icon}}
///   </div>
///   <div class="text-fd-card-foreground/70">
///   {{title}}
///   </div>
///   </div> {% endif %}
///    <div class="w-full relative" id="{{id}}">
///    <div
///      data-cdrm-copy-for="{{id}}"
///      role="button"
///      class="auto-codeblock-icon absolute z-1 top-2 right-2 transition-opacity duration-300"
///      onclick="copyCodeblockCode"
///    >
///    {{crate::output::html::icons::embedded_web_icons::EmbeddedIcon::Copy}}
///    </div>
///    <div class="[&>pre]:p-2! [&>pre]:rounded-bl [&>pre]:rounded-br [&>pre]:rounded-tl-none [&_pre]:rounded-tr-none [&>pre]:my-0 [&>pre]:text-sm">
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
        println!("Title: {:#?}", title);
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
