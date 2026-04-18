use askama::Template;

/// ```askama
/// <div class="border-primary! pl-2 border-l-[4px]">
/// {{self.children}}
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct BlockQuoteHtmlTemplate {
    pub children: String,
}
