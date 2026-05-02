use crate::{lang::lib::ui::ui_types::children::Children, parsers::markdown::table::table_cell_data::TableCellData};

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone)]
pub struct MarkdownTableCell {
    pub data: TableCellData,
}
