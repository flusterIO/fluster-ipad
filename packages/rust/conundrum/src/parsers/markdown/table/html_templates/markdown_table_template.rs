use askama::Template;

use crate::parsers::markdown::table::html_templates::{
    markdown_table_heading_cell_template::MarkdownTableHeadingCellTemplate,
    markdown_table_row_template::MarkdownTableRowTemplate,
};

/// ## Template (HTML)
///
/// ```askama
/// <table class="w-full h-fit">
/// <tr>
/// {% for heading_cell in self.heading_cells %}
/// {{heading_cell.render()?}}
/// {% endfor %}
/// </tr>
/// {% for row in self.rows %}
/// {{row.render()?}}
/// {% endfor %}
/// </table>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct MarkdownTableTemplate {
    pub heading_cells: Vec<MarkdownTableHeadingCellTemplate>,
    pub rows: Vec<MarkdownTableRowTemplate>,
}
