use conundrum::output::{
    auxillary::dictionary_page::dictionary_letter::DictionaryLetter,
    auxillary::dictionary_page::dictionary_page_template::{render_dictionary, DictionaryPageTemplate},
    parsing_result::dictionary_result::DictionaryEntryResult,
};

use conundrum::lang::runtime::state::conundrum_error_variant::{ConundrumModalResult, ConundrumResult};
use conundrum::parsers::markdown::code_block::dictionary::dictionary_entry_html_temp::DictionaryEntryHtmlTemplate;

#[uniffi::export(async_runtime = "tokio")]
pub async fn render_dictionary_page_to_html(entries: Vec<DictionaryEntryHtmlTemplate>) -> ConundrumResult<String> {
    render_dictionary(entries)
}
