use askama::Template;
use tw_merge::*;

use crate::{
    output::html::{
        web_specific_models::tailwind_class_representable::TailwindClassRepresentable,
        web_specific_traits::data_attribute_representable::DataAttributeRepresentable,
    },
    parsers::markdown::lists::task_list::task_list_item::task_list_completion_indicator::TaskListCompletionToken,
};

/// ## Template (HTML)
/// ```askama
/// {% if let Some(body) = self.body %}
/// <li class="w-full [&>*]:inline">
/// <div>
/// <input
/// {{self.status.as_html_data_attribute()}}
/// type="checkbox"
/// class="border {{self.status.as_tailwind_class()}}"
/// />
/// {{heading | safe}}
/// </div>
/// <div class="prose-sm [&>p]:mt-2">
/// {{body | safe}}
/// </div>
/// </li>
/// {% else %}
/// <li class="w-full [&>*]:inline">
/// <input
/// {{self.status.as_html_data_attribute()}}
/// type="checkbox"
/// class="border {{self.status.as_tailwind_class()}}"
/// />
/// {{heading | safe}}
/// </li>
/// {% endif %}
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct UnorderedTaskListItemHtmlTemplate {
    pub heading: String,
    pub body: Option<String>,
    pub status: TaskListCompletionToken,
}
