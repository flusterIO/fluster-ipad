use askama::Template;

use crate::lang::lib::ui::{
    components::media::image::image_html_templ::ImageHtmlTemplate, shared_props::sizable::SizablePropsGroup,
};

/// ## Template (HTML)
/// ```askama
/// <div
/// {% if let Some(sizable) = self.sizable %}
/// class="{{sizable.as_class(crate::lang::lib::ui::shared_props::sizable::SizablePropsOutputTarget::Image) | safe}}"
/// {% endif %}
/// >
/// {{img_templ.render()? | safe}}
/// <div class="w-full text-sm {{self.get_caption_text_alignment_class()}} [&>img]:w-full [&>img]:h-auto">
/// {{caption | safe}}
/// </div>
/// </div>
/// ```
#[derive(Template)]
#[template(in_doc = true, ext = "html")]
pub struct ImageHtmlTemplateWithCaption {
    pub img_templ: ImageHtmlTemplate,
    pub sizable: Option<SizablePropsGroup>,
    pub caption: String,
    pub caption_left: bool,
    pub caption_right: bool,
}

impl ImageHtmlTemplateWithCaption {
    pub fn get_caption_text_alignment_class(&self) -> String {
        if self.caption_left {
            String::from("text-left")
        } else if self.caption_right {
            String::from("text-right")
        } else {
            String::from("text-center")
        }
    }
}
