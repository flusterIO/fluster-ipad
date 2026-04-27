use askama::Template;


/// Template (HTML)
/// ```askama
/// <div class="text-5xl">
/// Table template here
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct MarkdownTableHtmlTemplate {
     
}
