use askama::Template;

/// ## Template (HTML)
///
/// ```askama
/// {{title}}: {{content}}
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct DictionaryEntryPlainTextTemplate {
    pub title: String,
    pub content: String,
}
