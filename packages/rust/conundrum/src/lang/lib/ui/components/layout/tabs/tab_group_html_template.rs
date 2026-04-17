use askama::Template;

use crate::lang::lib::ui::components::layout::tabs::tab_html_template::TabButtonHtmlTemplate;

/// ## Template (HTML)
///
/// ```askama
/// <div
///     class="cdrm-tab-group w-full bg-fd-card rounded-br rounded-bl relative overflow-x-hidden overflow-y-auto" data-cdrm-tab-group="{{tab_group_id}}"  >
///     <div class="w-full flex flex-row justify-start items-start flex-nowrap overflow-x-auto
///     overflow-y-none">
///     {% for tab in tabs %}
///        tab.render()
///     {% endfor %}
///     </div>
///     <div class="w-full bg-fd-card rounded-br rounded-bl relative overflow-x-hidden
///     overflow-y-auto" id="{{tab_group_id}}" data-cdrm-n-tabs="{{tabs.len()}}">
///         {{children}}
///     </div>
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct TabGroupHtmlTemplate {
    /// The _rendered_ children.
    pub children: String,
    pub tab_group_id: String,
    pub tabs: Vec<TabButtonHtmlTemplate>,
}
