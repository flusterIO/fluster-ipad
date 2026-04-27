use askama::Template;
use lexopt::Arg::Value;

pub struct OrderedListItemHtmlTemplate {
    pub heading: String,
    pub body: Option<String>,
}

/// Template (HTML)
///
/// ```askama
/// <div class="text-5xl">
/// Ordered Lst template here
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct OrderedListHtmlTemplate {
    pub items: Vec<OrderedListItemHtmlTemplate>,
}
