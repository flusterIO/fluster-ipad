use crate::parsers::markdown::table::markdown_table_heading_cell::MarkdownTableHeadingCell;

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone)]
pub struct MarkdownTableHeadingRow(Vec<MarkdownTableHeadingCell>);
