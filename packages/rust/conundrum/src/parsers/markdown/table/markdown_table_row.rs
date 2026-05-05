use winnow::Parser;

use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    parsers::{
        markdown::table::{markdown_table_cell::MarkdownTableCell, table_utility_parsers::table_row_parser_wrapper},
        parser_trait::{ConundrumParser, ConundrumParserWithParam},
    },
};

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone)]
pub struct MarkdownTableRow(Vec<MarkdownTableCell>);

impl ConundrumParserWithParam<usize, MarkdownTableRow> for MarkdownTableRow {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>,
                              col_count: usize)
                              -> ConundrumModalResult<MarkdownTableRow> {
        let res = table_row_parser_wrapper(MarkdownTableCell::parse_input_string, Some(col_count)).parse_next(input)?;
        Ok(MarkdownTableRow(res))
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_basic_table_heading_row() {
        let mut input = wrap_test_conundrum_content("| My cell here. | 2.1415 | 3 | My other cell |");
        let res =
            MarkdownTableRow::parse_input_string(&mut input, 4).expect("Parses table row without throwing an error.");

        println!("Res: {:#?}", res);
        assert!(res.0.len() == 4, "Returns the proper number of heading elements");
    }
}
