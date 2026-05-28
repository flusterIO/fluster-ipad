use askama::Template;

use crate::parsers::markdown::lists::task_list::task_list_item::task_list_item_html_templ::UnorderedTaskListItemHtmlTemplate;

/// ## Template (HTML)
///
/// ```askama
/// <ul class="w-full my-4 max-w-[1080px] list-disc list-inside">
/// {% for item in items %}
/// {{item.render()? | safe}}
/// {% endfor %}
/// </ul>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct UnorderedTaskListHtmlTemplate {
    pub items: Vec<UnorderedTaskListItemHtmlTemplate>,
}
