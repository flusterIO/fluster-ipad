use askama::Template;

use crate::lang::lib::ui::ui_types::emphasis::Emphasis;

/// ## Template (HTML)
/// ```askama
/// <span class="[&_*]:text-inherit px-[0.2rem] rounded-[4px] {{self.emphasis.to_background_color_classes() | safe}}">{{content | safe}}</span>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct HlHtmlTemplate {
    pub emphasis: Emphasis,
    pub content: String,
}
