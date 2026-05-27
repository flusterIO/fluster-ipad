use std::sync::Arc;

use winnow::Parser;

use crate::{
    lang::runtime::{
        state::conundrum_error_variant::ConundrumModalResult,
        traits::{
            conundrum_input::{ArcState, ConundrumInput},
            conundrum_template::ConundrumTemplateRepresentableWithParam,
        },
    },
    parsers::{
        markdown::table::{
            html_templates::markdown_table_heading_cell_template::MarkdownTableHeadingCellTemplate,
            markdown_table_cell::conundrum_content_markdown_table_cell, table_cell_data::TableCellData,
            table_types::TableCellAlignment,
        },
        parser_trait::ConundrumParser,
    },
};

#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct MarkdownTableHeadingCell {
    pub data: TableCellData,
}

impl ConundrumTemplateRepresentableWithParam<MarkdownTableHeadingCellTemplate, TableCellAlignment>
    for MarkdownTableHeadingCell
{
    fn to_template(&self,
                   state: ArcState,
                   params: TableCellAlignment)
                   -> ConundrumModalResult<MarkdownTableHeadingCellTemplate> {
        Ok(MarkdownTableHeadingCellTemplate { content: self.data.render(Arc::clone(&state))?,
                                              alignment: params })
    }
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
