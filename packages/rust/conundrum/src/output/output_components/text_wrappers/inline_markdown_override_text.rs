use crate::lang::lib::ui::ui_types::inline_markdown_override::InlineMarkdownOverride;

pub fn italic_text(content: &str) -> String {
    format!("_{}_", content)
}
pub fn bold_italic_text(content: &str) -> String {
    format!("**_{}_**", content)
}
pub fn bold_text(content: &str) -> String {
    format!("**{}**", content)
}
