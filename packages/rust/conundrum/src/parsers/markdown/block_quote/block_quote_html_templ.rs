use askama::Template;

/// ```askama
/// <blockquote class="cdrm-block-quote border-primary! not-prose pl-2 border-l-[4px] w-full mt-4">
/// {{self.children | safe}}
/// </blockquote>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct BlockQuoteHtmlTemplate {
    pub children: String,
}
