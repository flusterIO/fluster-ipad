use crate::lang::lib::ui::shared_props::sizable_option::SizableOption;
use askama::Template;
use tw_merge::*;

///  ## Template (HTML)
///
///  ```askama
///  <{{tag}} class="{{tw_merge!("[&>svg]:max-w-full [&>svg]:max-h-full",
/// self.sizable_classes.clone(), self.get_size_classes()) | safe}}"> {{emoji |
/// safe}}  </{{tag}}>
///  ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct EmojiHtmlTemplate {
    pub emoji: String,
    pub sizable_classes: String,
    pub size: SizableOption,
    /// Either 'div' or 'tag'.
    pub tag: String,
}

impl EmojiHtmlTemplate {
    pub fn get_size_classes(&self) -> String {
        match self.size {
            SizableOption::Small => "w-4 h-auto".to_string(),
            SizableOption::Smedium => "w-6 h-auto".to_string(),
            SizableOption::Medium => "w-12 h-auto".to_string(),
            SizableOption::Large => "w-32 h-auto".to_string(),
            SizableOption::Xl => "w-64 h-auto".to_string(),
            SizableOption::Xxl => "w-128 h-auto".to_string(),
            SizableOption::Fit => "w-full h-auto [&_svg]:object-fit".to_string(),
            SizableOption::Full => "w-full h-auto [&_svg]:object-fill".to_string(),
            SizableOption::None => "w-3 h-auto".to_string(),
        }
    }
}
