use askama::Template;

use crate::parsers::{
    conundrum::logic::number::conundrum_number::ConundrumNumber, markdown::table::table_types::TableCellAlignment,
};

#[derive(Template)]
pub enum MarkdownTableCellTemplate {
    /// ## Template (HTML)
    ///
    /// ```askama
    /// <th>{{self.0 | safe}}</th>
    /// ```
    #[template(ext = "html", in_doc = true)]
    Heading(String, TableCellAlignment),
    /// ## Template (HTML)
    ///
    /// ```askama
    /// <td>{{self.0 | safe}}</td>
    /// ```
    #[template(ext = "html", in_doc = true)]
    Content(String, TableCellAlignment),
    /// ## Template (HTML)
    ///
    /// ```askama
    /// <td>{{self.0 | safe}}</td>
    /// ```
    #[template(ext = "html", in_doc = true)]
    Numeric(ConundrumNumber, TableCellAlignment),
}
