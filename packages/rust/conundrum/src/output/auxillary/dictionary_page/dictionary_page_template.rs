use crate::lang::runtime::state::conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult};
use crate::lang::runtime::state::conundrum_error_variant::ConundrumResult;
use crate::lang::runtime::state::conundrum_error::{ConundrumError};
use crate::output::{
    parsing_result::dictionary_result::DictionaryEntryResult,
};
use crate::parsers::markdown::code_block::dictionary::dictionary_entry_html_temp::DictionaryEntryHtmlTemplate;
use askama::Template;

/// ## Template (HTML)
/// ```askama
/// <div class="w-full max-w-[1080px]">
/// {% for entry in self.data %}
/// <div class="flex flex-col justify-start items-start">
///  {{entry.render()? | safe}}
/// </div>
/// {% endfor %}
/// </div>
/// ```
#[derive(Template)]
#[template(in_doc = true, ext = "html")]
pub struct DictionaryPageTemplate {
    /// TODO: Install that indexmap package or whatever it was called so you can
    /// ensure the order when this is iterated.
    data: Vec<DictionaryEntryHtmlTemplate>,
}

pub fn sort_entries(mut vec: Vec<DictionaryEntryHtmlTemplate>) -> Vec<DictionaryEntryHtmlTemplate> {
    vec.sort_by(|a, b| a.label.cmp(&b.label));
    vec
}

pub fn render_dictionary(mut entries: Vec<DictionaryEntryHtmlTemplate>) -> ConundrumResult<String> {
    let sorted = sort_entries(entries);
    let templ = DictionaryPageTemplate {
        data: sorted
    };
    templ.render().map_err(|e| {
                eprintln!("Error: {:#?}", e);
                ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error())
            })
}
