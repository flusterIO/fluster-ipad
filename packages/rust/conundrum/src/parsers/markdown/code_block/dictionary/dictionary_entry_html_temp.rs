use crate::output::html::glue::glue_code_rust::cdrm_web_events::ConundrumWebEvents;
use askama::Template;

/// ## Template (HTML)
///
/// ```askama
/// <div class="cdrm-dict-entry w-full my-3 max-w-[1080px] font-serif">
/// <div {% if let Some(note_id) = self.note_id  %} data-cdrm-noteid="{{note_id}}" {% endif %} class="[&_p]:text-xl font-medium tracking-tight text-foreground decoration-none underline-none {% if self.note_id.is_some() %} cursor-pointer{% endif %}" onclick="window.conundrum.onDictionaryEntryClick(event)">
/// {{label | safe}}
/// </div>
/// <div class="w-full pl-4">
/// {{body | safe}}
/// </div>
/// </div>
/// ```
#[derive(Template, uniffi::Record)]
#[template(ext = "html", in_doc = true)]
pub struct DictionaryEntryHtmlTemplate {
    pub label: String,
    pub body: String,
    /// If note_id is Some, the link will have a class of `cursor-pointer`
    /// applied and a data attribute to make the div more easily used for
    /// navigatiion.
    pub note_id: Option<String>,
}
