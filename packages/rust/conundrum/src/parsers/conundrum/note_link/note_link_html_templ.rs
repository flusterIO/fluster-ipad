use askama::Template;

/// ```askama
/// <a data-note-id-referencing="{{note_id}}" role="button" onclick="handleNoteLinkClick">{{children}}</a>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct NoteLinkHtmlTemplate {
    pub children: String,
    pub note_id: String,
}
