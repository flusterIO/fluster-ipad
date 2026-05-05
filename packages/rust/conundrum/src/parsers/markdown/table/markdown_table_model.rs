use crate::{
    lang::runtime::{
        state::conundrum_error_variant::ConundrumModalResult,
        traits::{
            conundrum_input::ConundrumInput, fluster_component_result::ConundrumComponentResult,
            html_js_component_result::HtmlJsComponentResult, plain_text_component_result::PlainTextComponentResult,
        },
    },
    parsers::{
        markdown::table::{
            html_templates::markdown_table_template::MarkdownTableTemplate,
            markdown_table_alignment_row::MarkdownTableAlignmentRow,
            markdown_table_heading_row::MarkdownTableHeadingRow, markdown_table_row::MarkdownTableRow,
        },
        parser_trait::{ConundrumParser, ConundrumParserWithParam},
    },
};
use serde::Serialize;
use winnow::{Parser, combinator::repeat, stream::Stream};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct MarkdownTable {
    pub heading: MarkdownTableHeadingRow,
    pub alignment: MarkdownTableAlignmentRow,
    pub rows: Vec<MarkdownTableRow>,
}

impl PlainTextComponentResult for MarkdownTable {
    fn to_plain_text(&self,
                     res: crate::lang::runtime::traits::conundrum_input::ArcState)
                     -> ConundrumModalResult<String> {
        todo!()
    }
}

impl HtmlJsComponentResult for MarkdownTable {
    fn to_html_js_component(&self,
                            res: crate::lang::runtime::traits::conundrum_input::ArcState)
                            -> ConundrumModalResult<String> {
        todo!()
    }
}

impl ConundrumComponentResult for MarkdownTable {
    fn to_conundrum_component(&self,
                              res: crate::lang::runtime::traits::conundrum_input::ArcState)
                              -> ConundrumModalResult<String> {
        todo!()
    }
}

impl ConundrumParser<MarkdownTable> for MarkdownTable {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownTable> {
        let start = input.input.checkpoint();
        let (heading, col_count): (MarkdownTableHeadingRow, usize) =
            MarkdownTableHeadingRow::parse_input_string.parse_next(input).inspect_err(|_| {
                                                                              input.input.reset(&start);
                                                                          })?;

        let alignment = MarkdownTableAlignmentRow::parse_input_string(input, col_count).inspect_err(|_| {
                                                                                           input.input.reset(&start);
                                                                                       })?;

        let rows: Vec<MarkdownTableRow> = repeat(1.., |nested_input: &mut ConundrumInput| {
                                              MarkdownTableRow::parse_input_string(nested_input, col_count)
                                          }).parse_next(input)
                                            .inspect_err(|_| {
                                                input.input.reset(&start);
                                            })?;

        Ok(MarkdownTable { heading,
                           alignment,
                           rows })
    }

    fn matches_first_char(char: char) -> bool {
        char == '|'
    }
}

impl MarkdownTable {
    pub fn to_template(&self) -> MarkdownTableTemplate {}
}
