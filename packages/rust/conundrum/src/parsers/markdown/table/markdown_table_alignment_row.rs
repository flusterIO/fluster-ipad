use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    parsers::{
        markdown::table::{
            markdown_table_alignment_cell::MarkdownTableAlignmentCell,
            table_types::{TableCellAlignment, TableCellAlignmentMap},
        },
        parser_components::white_space_delimited::space_or_tab_delimited,
        parser_trait::{ConundrumParser, ConundrumParserWithParam},
    },
};

use dashmap::DashMap;
use winnow::{
    Parser,
    combinator::{delimited, separated},
};

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone)]
pub struct MarkdownTableAlignmentRow(Vec<MarkdownTableAlignmentCell>);

impl ConundrumParserWithParam<usize, MarkdownTableAlignmentRow> for MarkdownTableAlignmentRow {
    fn parse_input_string(input: &mut ConundrumInput,
                          col_count: usize)
                          -> ConundrumModalResult<MarkdownTableAlignmentRow> {
        let res: Vec<MarkdownTableAlignmentCell> =
            delimited('|',
                      separated(col_count,
                                space_or_tab_delimited(MarkdownTableAlignmentCell::parse_input_string),
                                '|'),
                      '|').parse_next(input)?;
        Ok(MarkdownTableAlignmentRow(res))
    }

    // fn matches_first_char(char: char) -> bool {
    //     char == '|'
    // }
}

impl MarkdownTableAlignmentRow {
    pub fn to_col_map(&self) -> TableCellAlignmentMap {
        let m: DashMap<usize, TableCellAlignment> = DashMap::new();
        self.0.iter().enumerate().for_each(|(idx, cell)| {
                                     m.insert(idx, cell.to_cell_alignment_enum());
                                 });
        TableCellAlignmentMap::new(m)
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    fn parses_input(input: &str, target_length: usize) {
        let mut input = wrap_test_conundrum_content(input);
        let res = MarkdownTableAlignmentRow::parse_input_string(&mut input, target_length).expect("Parses input as expected.");
        println!("Length: {}", res.0.len());
        assert!(res.0.len() == target_length, "Finds the proper number of cells.")
    }

    #[test]
    fn parses_simple_table_alignment_row() {
        parses_input("| :---- |------|   -----: | --|", 4);
        parses_input("|:-----------------|:-----------------: |-----------------:|---------------------|---------------------|",
                     5);
    }
}
