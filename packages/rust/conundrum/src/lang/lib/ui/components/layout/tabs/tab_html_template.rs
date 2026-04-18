use askama::Template;

use crate::parsers::conundrum::logic::bool::boolean::ConundrumBoolean;

/// ## Template
/// ```askama
/// <div id="{{get_content_id()}}" class="cdrm-tab-group-tab w-full absolute px-4 py-3">
/// {{self.content}}
/// <div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct TabContentTemplate {
    pub content: String,
    pub idx: u8,
    pub group_id: String,
}

impl TabContentTemplate {
    pub fn get_content_id(&self) -> String {
        format!("{}-{}", self.group_id, self.idx)
    }
}

/// ## Template (HTML)
///
/// ```askama
/// <div
///     class="cdrm-tab-group-tab w-full absolute px-4 py-3{% if idx > 0 %} translate-x-full {% endif %}"
///     id="{{tab_group_id}}-{{idx}}"
///     onclick="handleTabClick"
///     data-cdrm-tab-idx="{{idx}}"
///     data-cdrm-group-id="{{tab_group_id}}"
/// >
/// {{btn_children}}
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct TabButtonHtmlTemplate {
    /// The _rendered_ children of the button, not the tab itself.
    pub btn_children: String,
    pub tab_group_id: String,
    pub idx: u8,
    /// Set to true to make this the initial tab.
    pub initial: ConundrumBoolean,
    /// The rendered children of the tab itself, not the tab button.
    pub tab_children: TabContentTemplate,
}
