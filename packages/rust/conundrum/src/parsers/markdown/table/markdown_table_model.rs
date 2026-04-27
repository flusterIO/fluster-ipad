use typst::foundations::PlainText;

use crate::{
    lang::runtime::{
        state::conundrum_error_variant::ConundrumModalResult,
        traits::{
            conundrum_input::ConundrumInput, fluster_component_result::ConundrumComponentResult,
            html_js_component_result::HtmlJsComponentResult, plain_text_component_result::PlainTextComponentResult,
        },
    },
    parsers::{markdown::table::markdown_table_row::MarkdownTableRow, parser_trait::ConundrumParser},
};

pub struct MarkdownTable {
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
        todo!()
    }

    fn matches_first_char(char: char) -> bool {
        char == '|'
    }
}
