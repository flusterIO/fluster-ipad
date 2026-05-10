use askama::Template;

use crate::parsers::markdown::table::table_types::TableCellAlignment;

/// ## Template (HTML)
///
/// ```askama
/// <th class="{{self.alignment | safe}}">{{content | safe}}</th>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct MarkdownTableHeadingCellTemplate {
    pub content: String,
    pub alignment: TableCellAlignment,
}
