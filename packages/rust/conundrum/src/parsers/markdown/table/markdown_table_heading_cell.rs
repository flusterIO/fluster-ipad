use crate::parsers::markdown::table::table_cell_data::TableCellData;

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone)]
pub struct MarkdownTableHeadingCell {
    pub data: TableCellData,
}
