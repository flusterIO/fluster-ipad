use askama::Template;
use devicons::{FileIcon, Theme};

use crate::{
    output::html::dom::dom_id::DOMId, parsers::markdown::code_block::supported_languages::SupportedCodeBlockSyntax,
};

/// ## Template (HTML)
///
/// ```askama
/// <div>
/// Here
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct CodeBlockHTMLTemplate {
    pub id: DOMId,
    pub code: String,
    pub title: Option<String>,
    // pub file_icon: FileIcon,
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
                                title /* file_icon */ }
    }
}
