use askama::Template;

/// ## Template (HTML)
///
/// ```askama
/// ##### {{title}}
///
/// {{content}}
/// ```
#[derive(Template)]
#[template(ext = "txt", escape = "none", in_doc = true)]
pub struct DictionaryEntryMarkdownTemplate {
    pub title: String,
    pub content: String,
}
