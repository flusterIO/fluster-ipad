use askama::Template;

/// ```askama
/// <style>
/// {{css}}
/// </style>
/// {{body}}
/// <script>
/// {{js}}
/// </script>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct DocumentTemplate {
    pub body: String,
    pub js: String,
    pub css: String,
}
