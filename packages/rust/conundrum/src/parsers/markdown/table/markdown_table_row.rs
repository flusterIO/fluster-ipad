use crate::parsers::markdown::table::{
    markdown_table_alignment_row::MarkdownTableAlignmentRow, markdown_table_cell::MarkdownTableCell,
    markdown_table_heading_row::MarkdownTableHeadingRow,
};

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone)]
pub struct MarkdownTableRow {
    pub heading: MarkdownTableHeadingRow,
    pub alignment: MarkdownTableAlignmentRow,
    pub cells: Vec<MarkdownTableCell>,
}
