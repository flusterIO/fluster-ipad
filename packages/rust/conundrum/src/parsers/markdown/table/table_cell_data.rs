use crate::{
    lang::lib::ui::ui_types::children::Children, parsers::conundrum::logic::number::conundrum_number::ConundrumNumber,
};

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum TableCellData {
    Heading(Children),
    Conundrum(Children),
    Numeric(ConundrumNumber),
}
