use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    parsers::{
        markdown::table::{
            markdown_table_heading_cell::MarkdownTableHeadingCell, table_utility_parsers::table_row_parser_wrapper,
        },
        parser_trait::ConundrumParser,
    },
};
use winnow::Parser;

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone)]
pub struct MarkdownTableHeadingRow(Vec<MarkdownTableHeadingCell>);

impl ConundrumParser<MarkdownTableHeadingRow> for MarkdownTableHeadingRow {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<MarkdownTableHeadingRow> {
        let res = table_row_parser_wrapper(MarkdownTableHeadingCell::parse_input_string).parse_next(input)?;
        Ok(MarkdownTableHeadingRow(res))
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
    fn parses_basic_table_heading_row() {
        let mut input = wrap_test_conundrum_content("| My heading here | Some other heading | here | here | here |");
        let res =
            MarkdownTableHeadingRow::parse_input_string.parse_next(&mut input)
                                                       .expect("Parses table heading row without throwing an error.");

        println!("Res: {:#?}", res);
        assert!(res.0.len() == 5, "Returns the proper number of heading elements");
    }
}
