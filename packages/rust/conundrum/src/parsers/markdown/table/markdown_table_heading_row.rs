use std::sync::Arc;

use crate::{
    lang::runtime::{
        state::conundrum_error_variant::ConundrumModalResult,
        traits::{
            conundrum_input::{ArcState, ConundrumInput},
            conundrum_template::HTMLTemplatePossiblyRepresentableWithParam,
        },
    },
    parsers::{
        markdown::table::{
            html_templates::markdown_table_heading_cell_template::MarkdownTableHeadingCellTemplate,
            markdown_table_heading_cell::MarkdownTableHeadingCell, table_types::TableCellAlignmentMap,
            table_utility_parsers::table_row_parser_wrapper,
        },
        parser_trait::ConundrumParser,
    },
};
use winnow::Parser;

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct MarkdownTableHeadingRow(Vec<MarkdownTableHeadingCell>);

impl ConundrumParser<(MarkdownTableHeadingRow, usize)> for MarkdownTableHeadingRow {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>)
                              -> ConundrumModalResult<(MarkdownTableHeadingRow, usize)> {
        let res = table_row_parser_wrapper(MarkdownTableHeadingCell::parse_input_string, None).parse_next(input)?;
        Ok((MarkdownTableHeadingRow(res.clone()), res.len()))
    }

    fn matches_first_char(char: char) -> bool {
        char == '|'
    }
}

impl MarkdownTableHeadingRow {
    pub fn to_heading_cell_templates(&self,
                                     state: ArcState,
                                     alignment_map: &TableCellAlignmentMap)
                                     -> ConundrumModalResult<Vec<MarkdownTableHeadingCellTemplate>> {
        let items =
            self.0
                .iter()
                .enumerate()
                .map(|(idx, item)| item.to_template(Arc::clone(&state), alignment_map.get_col_alignment_by_idx(&idx)))
                .collect::<ConundrumModalResult<Vec<MarkdownTableHeadingCellTemplate>>>()?;
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_basic_table_heading_row() {
        let mut input = wrap_test_conundrum_content("| My heading here | Some other heading | here | here | here |");
        let (res, n): (MarkdownTableHeadingRow, usize) =
            MarkdownTableHeadingRow::parse_input_string.parse_next(&mut input)
                                                       .expect("Parses table heading row without throwing an error.");

        assert!(res.0.len() == 5, "Returns the proper number of heading elements");

        assert!(n == 5, "Returns the proper number of heading elements in the tuple.");
    }

    #[test]
    fn parses_only_single_row() {
        let mut input = wrap_test_conundrum_content(
                                                    r#"| One | Two | Three |
| ---- | ---- | ---- |
| Here | Here | And Here |"#,
        );

        let (r, n) = MarkdownTableHeadingRow::parse_input_string.parse_next(&mut input)
                                                                .expect("Parses table without throwing an error.");

        println!("Input: {}", input.input);
        assert!(
                input.input
                == r#"| ---- | ---- | ---- |
| Here | Here | And Here |"#,
                "Leaves the proper content behind"
        );
    }
}
