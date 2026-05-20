use askama::Template;

use crate::lang::lib::ui::components::media::image::{
    image_html_templ::ImageHtmlTemplate, image_html_templ_with_caption::ImageHtmlTemplateWithCaption,
};

#[derive(Template)]
pub enum AnyImageHtmlTemplate {
    #[template(source = "{{self.0.render()? | safe}}", ext = "html")]
    WithCaption(ImageHtmlTemplateWithCaption),
    #[template(source = "{{self.0.render()? | safe}}", ext = "html")]
    NoCaption(ImageHtmlTemplate),
}
