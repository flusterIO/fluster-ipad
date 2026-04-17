use askama::Template;

/// ## Template
/// ```askama
/// <div class="cdrm-tab-group-tab w-full absolute px-4 py-3">
/// {{self.0}}
/// <div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct TabContentTemplate(String);

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
/// {{children}}
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct TabButtonHtmlTemplate {
    /// The _rendered_ children of the button, not the tab itself.
    pub children: String,
    pub tab_group_id: String,
    pub idx: u8,
    /// Set to true to make this the initial tab.
    pub initial: bool,
    /// The rendered children of the tab itself, not the tab button.
    pub tab_content: TabContentTemplate,
}
