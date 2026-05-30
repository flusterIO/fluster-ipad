use askama::Template;

use crate::lang::lib::ui::components::documentation::emoji::emoji_data::EmojiData;

/// ## Template (HTML)
///
/// ```askama
/// <div class="w-full h-fit flex flex-col justify-center items-center gap-y-4">
/// <input placeholder="Search emojis..." onchange="window.conundrum.onEmojiDocsInputChange(event)" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-base shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 @[768px]/mdx:text-sm"/>
/// <div id="emoji-docs-content" class="w-full grid grid-cols-1 @[350px]/mdx:grid-cols-2 @[450px]/mdx:grid-cols-3 @[650px]/mdx:grid-cols-4 gap-2">
/// {% for item in self.items %}
/// {{item.render()? | safe}}
/// {% endfor %}
/// </div>
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct EmojiDocsHtmlTemplate {
    pub items: Vec<EmojiData>,
}
