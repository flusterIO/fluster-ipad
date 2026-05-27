use askama::Template;

use crate::parsers::markdown::lists::ordered::ordered_list_item::ordered_list_item_html_templ::OrderedListItemHtmlTemplate;

/// ## Template (HTML)
///
/// ```askama
/// <ol class="w-full max-w-[1080px] list-decimal list-numeric list-dec">
/// {% for item in items %}
/// {{item.render()? | safe}}
/// {% endfor %}
/// </ol>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct OrderedListHtmlTemplate {
    pub items: Vec<OrderedListItemHtmlTemplate>,
}
