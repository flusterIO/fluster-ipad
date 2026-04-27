use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    parsers::{markdown::table::markdown_table_cell::MarkdownTableCell, parser_trait::ConundrumParser},
};

pub struct MarkdownTableRow {
    pub cells: Vec<MarkdownTableCell>,
}

impl ConundrumParser<MarkdownTableRow> for MarkdownTableRow {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownTableRow> {
        todo!()
    }

    fn matches_first_char(char: char) -> bool {
        char == '|'
    }
}
