use crate::{lang::lib::ui::ui_types::children::Children, parsers::conundrum::logic::number::conundrum_number::ConundrumNumber};

pub enum TableCellDataType {
    General(Children),
    DataPoint(ConundrumNumber)
}


pub struct MarkdownTableCell {
    pub content: TableCellDataType,
}
