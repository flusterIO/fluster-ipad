use crate::lang::lib::ui::{
    components::media::image::image_html_templ::ImageHtmlTemplate, shared_props::sizable::SizablePropsGroup,
};
use askama::Template;
use tw_merge::*;

/// ## Template (HTML)
///
/// ```askama
/// <div
/// class="{{tw_merge!("w-fit flex flex-col justify-center items-center gap-y-2 overflow-hidden", self.sizable.as_class(crate::lang::lib::ui::shared_props::sizable::SizablePropsOutputTarget::Image)) | safe}}"
/// >
/// <img
/// src="{{self.img_templ.src | safe}}"
/// class="{{tw_merge!("max-h-full max-w-full w-full h-auto object-contain", self.sizable.as_class(crate::lang::lib::ui::shared_props::sizable::SizablePropsOutputTarget::ImageNested), match self.cover {
/// true => "object-cover",
/// false => "object-contain"
/// }, self.img_templ.get_object_fit_class()) | safe}}"
/// id="{{self.img_templ.id | safe}}"
/// ></img>
/// <div class="w-full text-sm {{self.get_caption_text_alignment_class()}} [&>img]:w-full [&>img]:h-auto">
/// {{caption | safe}}
/// </div>
/// </div>
/// ```
#[derive(Template)]
#[template(in_doc = true, ext = "html")]
pub struct ImageHtmlTemplateWithCaption {
    pub img_templ: ImageHtmlTemplate,
    pub sizable: SizablePropsGroup,
    pub caption: String,
    pub caption_left: bool,
    pub caption_right: bool,
    pub cover: bool,
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
