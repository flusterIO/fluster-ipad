use askama::Template;

/// ## Template (HTML)
///
/// ```askama
/// <th>{{content | safe}}</th>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct MarkdownTableHeadingCellTemplate {
    pub content: String,
}
