use askama::Template;

/// ## Template (HTML)
///
/// ```askama
/// <{{tag}} class="cursor-pointer">
/// {{idx}}
/// </{{tag}}>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct EquationRefHtmlTempl {
    /// The ***ONE*** based index to be displayed to the user.
    pub idx: u32,
    pub tag: String,
}
