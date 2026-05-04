use winnow::Parser;

use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    parsers::{
        markdown::table::{markdown_table_cell::conundrum_content_markdown_table_cell, table_cell_data::TableCellData},
        parser_trait::ConundrumParser,
    },
};

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone)]
pub struct MarkdownTableHeadingCell {
    pub data: TableCellData,
}

impl ConundrumParser<MarkdownTableHeadingCell> for MarkdownTableHeadingCell {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownTableHeadingCell> {
        let data = conundrum_content_markdown_table_cell.parse_next(input)?;
        Ok(MarkdownTableHeadingCell { data })
    }

    fn matches_first_char(_: char) -> bool {
        true
    }
}
