use crate::{
    lang::lib::ui::ui_types::children::Children, parsers::conundrum::logic::number::conundrum_number::ConundrumNumber,
};

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum TableCellData {
    /// Headings are made unique to allow for customizablity when the
    /// programming layer finally makes it onboard, but they're really the same
    /// as the body cells apart from the ablity to accept numeric content.
    Heading(Children),
    Conundrum(Children),
    Numeric(ConundrumNumber),
}
