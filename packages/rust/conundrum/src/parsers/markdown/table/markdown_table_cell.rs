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
        markdown::table::table_cell_data::TableCellData,
        parser_trait::ConundrumParser,
    },
};

use winnow::{
    Parser,
    combinator::{alt, repeat},
};

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone)]
pub struct MarkdownTableCell {
    pub data: TableCellData,
}

pub fn numeric_markdown_table_cell(input: &mut ConundrumInput) -> ConundrumModalResult<TableCellData> {
    ConundrumNumber::parse_conundrum.map(TableCellData::Numeric).parse_next(input)
}

pub fn conundrum_content_markdown_table_cell(input: &mut ConundrumInput) -> ConundrumModalResult<TableCellData> {
    let children: Vec<ParsedElement> = repeat(0..,
                                              parse_inline_element.verify(|em| match em {
                                                                      ParsedElement::Text(c) => c != &String::from("|"),
                                                                      _ => true,
                                                                  })).parse_next(input)?;
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
