use askama::Template;
use serde::Serialize;
use tabled::Tabled;

/// ## Template (HTML)
///
/// ```askama
/// <div class="w-full h-full grid grid-cols-1 grid-rows-[1fr_auto]" onclick="window.conundrum.copyString('{{name}}')">
/// <div class="flex flex-col justify-center items-center w-full h-full [&>svg]:max-h-[120px] [&>svg]:w-auto">{{svg | safe}}</div>
/// <div class="w-full text-center flex flex-col justify-center items-center text-sm text-fd-card-foreground/80">{{name | safe}}</div>
/// </div>
/// ```
#[typeshare::typeshare]
#[derive(Debug, Tabled, Serialize, uniffi::Record, Clone, Template)]
#[template(ext = "html", in_doc = true)]
pub struct EmojiData {
    pub name: String,
    pub svg: String,
}

impl EmojiData {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| String::from("{}"))
    }
}
