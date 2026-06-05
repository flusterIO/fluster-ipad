use askama::Template;

/// ## Template (HTML)
///
/// ```askama
/// <div class="w-full border rounded p-4 bg-fd-card text-fd-card-foreground my-8 border-primary/40 inline-block [&>.cdrm-heading-container]:mt-0 overflow-x-auto overflow-y-hidden">
///  {{children | safe}}
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct DocumentationContainerHtmlTemplate {
    pub children: String,
}
