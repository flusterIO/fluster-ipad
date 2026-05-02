use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    parsers::{
        markdown::table::markdown_table_alignment_cell::MarkdownTableAlignmentCell,
        parser_components::white_space_delimited::{space_or_tab_delimited, white_space_delimited},
        parser_trait::ConundrumParser,
    },
};

use winnow::{
    Parser,
    combinator::{delimited, separated},
};

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone)]
pub struct MarkdownTableAlignmentRow(Vec<MarkdownTableAlignmentCell>);

impl ConundrumParser<MarkdownTableAlignmentRow> for MarkdownTableAlignmentRow {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownTableAlignmentRow> {
        let res: Vec<MarkdownTableAlignmentCell> =
            delimited('|',
                      separated(1.., space_or_tab_delimited(MarkdownTableAlignmentCell::parse_input_string), '|'),
                      '|').parse_next(input)?;
        Ok(MarkdownTableAlignmentRow(res))
    }

    fn matches_first_char(char: char) -> bool {
        char == '|'
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_simple_table_alignment_row() {
        let mut input = wrap_test_conundrum_content("| :---- |------|   -----: | --|");
        let res = MarkdownTableAlignmentRow::parse_input_string(&mut input).expect("Parses input as expected.");
        assert!(res.0.len() == 4, "Finds the proper number of cells.")
        // assert_eq!(result, 4);
    }
}
