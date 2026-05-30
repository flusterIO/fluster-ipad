use std::collections::HashMap;

use crate::output::{
    auxillary::dictionary_page::dictionary_letter::DictionaryLetter,
    parsing_result::dictionary_result::DictionaryEntryResult,
};
use crate::lang::{
    runtime::state::conundrum_error_variant::ConundrumResult,
};
use askama::Template;

/// ## Template (HTML)
/// ```askama
/// <div class="grid grid-cols-[auto_1fr] w-full max-w-[1080px]">
/// {% for (dictionaryLetter, items) in self.data %}
/// <div class="flex flex-col justify-start items-start">
/// {{dictionaryLetter}}
/// </div>
/// <div>
/// {% for item in items %}
/// <div>
/// <div>{{item.label}}</div>
/// <div>{{item.body}}</div>
/// </div>
/// {% endfor %}
/// </div>
/// {% endfor %}
/// </div>
/// ```
#[derive(Template)]
#[template(in_doc = true, ext = "html")]
pub struct DictionaryPageTemplate {
    /// TODO: Install that indexmap package or whatever it was called so you can
    /// ensure the order when this is iterated.
    data: HashMap<DictionaryLetter, Vec<DictionaryEntryResult>>,
}

pub fn render_dictionary(entries: Vec<DictionaryEntryResult>) -> ConundrumResult<String> {
    Ok(String::from("FUCKKKKK THE RUST ANALYZER. THIS THING FUCKING SUCKS."))
}


