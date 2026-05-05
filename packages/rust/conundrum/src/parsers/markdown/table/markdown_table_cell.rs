use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::ui_types::children::Children,
        runtime::{
            parse_inline_elements::parse_inline_element, state::conundrum_error_variant::ConundrumModalResult,
            traits::conundrum_input::ConundrumInput,
        },
    },
    parsers::{
        conundrum::{conundrum_logic_parser::ConundrumLogicParser, logic::number::conundrum_number::ConundrumNumber},
        markdown::table::{
            table_cell_data::TableCellData, table_utility_parsers::terminating_whitespace_and_table_separator,
        },
        parser_components::consume_white_space::consume_linear_space,
        parser_trait::ConundrumParser,
    },
};

use winnow::{
    Parser,
    combinator::{alt, repeat_till},
    stream::Stream,
};

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone)]
pub struct MarkdownTableCell {
    pub data: TableCellData,
}

pub fn numeric_markdown_table_cell(input: &mut ConundrumInput) -> ConundrumModalResult<TableCellData> {
    let start = input.input.checkpoint();

    consume_linear_space(0..).parse_next(input).inspect_err(|_| {
                                                    input.input.reset(&start);
                                                })?;

    let n = ConundrumNumber::parse_conundrum.parse_next(input).inspect_err(|_| {
                                                                   input.input.reset(&start);
                                                               })?;
    terminating_whitespace_and_table_separator.parse_next(input).inspect_err(|_| {
                                                                     input.input.reset(&start);
                                                                 })?;
    Ok(TableCellData::Numeric(n))
}

pub fn conundrum_content_markdown_table_cell(input: &mut ConundrumInput) -> ConundrumModalResult<TableCellData> {
    let start = input.input.checkpoint();

    consume_linear_space(0..).parse_next(input).inspect_err(|_| {
                                                    input.input.reset(&start);
                                                })?;
    let (children, _): (Vec<ParsedElement>, ()) =
        repeat_till(1.., parse_inline_element, terminating_whitespace_and_table_separator.void()).parse_next(input)
                                                                                                 .inspect_err(|_| {
                                                                                                     input.input
                                                                                                        .reset(&start);
                                                                                                 })?;

    Ok(TableCellData::Conundrum(Children(children)))
}

impl ConundrumParser<MarkdownTableCell> for MarkdownTableCell {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<MarkdownTableCell> {
        let data = alt((numeric_markdown_table_cell, conundrum_content_markdown_table_cell)).parse_next(input)?;
        Ok(MarkdownTableCell { data })
    }

    fn matches_first_char(_: char) -> bool {
        true
    }
}
