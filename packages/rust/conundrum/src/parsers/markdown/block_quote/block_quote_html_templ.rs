use askama::Template;

/// ```askama
/// <div class="border-primary! pl-2 border-l-[4px] w-full mt-4">
/// {{self.children | safe}}
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct BlockQuoteHtmlTemplate {
    pub children: String,
}
