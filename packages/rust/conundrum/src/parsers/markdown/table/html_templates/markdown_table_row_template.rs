use askama::Template;

use crate::parsers::markdown::table::html_templates::markdown_table_cell_template::MarkdownTableCellTemplate;

/// ## Template (HTML)
///
/// ```askama
/// <tr>
/// {% for cell in self.cells %}
/// {{cell.render()? | safe}}
/// {% endfor %}
/// </tr>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct MarkdownTableRowTemplate {
    pub cells: Vec<MarkdownTableCellTemplate>,
}
