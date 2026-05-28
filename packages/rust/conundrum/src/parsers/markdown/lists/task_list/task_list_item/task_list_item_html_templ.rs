use askama::Template;

use crate::{
    lang::runtime::traits::icon_representable::IconRepresentable,
    output::html::{
        web_specific_models::tailwind_class_representable::TailwindClassRepresentable,
        web_specific_traits::data_attribute_representable::DataAttributeRepresentable,
    },
    parsers::markdown::lists::task_list::task_list_item::task_list_completion_indicator::TaskListCompletionToken,
};

/// ## Template (HTML)
/// ```askama
/// {% if let Some(body) = self.body %}
/// <li class="cdrm-task-list w-full [&>*]:inline bg-fd-card text-fd-card-foreground p-2 border rounded [&>.cdrm-task-list]:border-none [&>.cdrm-task-list]:bg-transparent! my-1 grid grid-cols-[auto_1fr] gap-x-2">
/// <div class="w-fit flex flex-col justify-start items-start py-1">
/// <span
/// {{self.status.as_html_data_attribute()}}
/// type="checkbox"
/// class="w-4 h-4 rounded-sm border shadow focus-visible:outline-none focus-visible:ring-1 flex! flex-col justify-center items-center text-sm place-self-center [&>*]:leading-none font-lucide text-[10px] {{self.status.as_tailwind_class()}}"
/// >
/// {{status.as_icon()}}
/// </span>
/// </div>
/// <div class="w-full flex flex-col justify-start items-start">
/// <div>
/// {{heading | safe}}
/// </div>
/// <div class="prose-sm [&>p]:mt-2">
/// {{body | safe}}
/// </div>
/// </div>
/// </li>
/// {% else %}
/// <li class="cdrm-task-list w-full [&>*]:inline bg-fd-card text-fd-card-foreground p-2 border rounded [&>.cdrm-task-list]:border-none [&>.cdrm-task-list]:bg-transparent! my-1 grid grid-cols-[auto_1fr] gap-x-2">
/// <span
/// {{self.status.as_html_data_attribute()}}
/// type="checkbox"
/// class="w-4 h-4 rounded-sm border shadow focus-visible:outline-none focus-visible:ring-1 flex! flex-col justify-center items-center text-sm place-self-center font-lucide text-[10px] {{self.status.as_tailwind_class()}}"
/// >
/// {{status.as_icon()}}
/// </span>
/// <span>
/// {{heading | safe}}
/// </span>
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
