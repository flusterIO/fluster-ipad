use askama::Template;

/// ## Template (HTML)
///
/// ```askama
/// <div class="cdrm-math cdrm-math-block flex justify-center items-center my-6 no-scrollbar overflow-x-auto overflow-y-hidden relative">
/// {{content | safe}}
/// <div class="cdrm-equation-label absolute right-0 top-[50%] translate-y-[-50%] text-sm font-math opacity-60{% if self.was_id %} with-id{% endif %}">
/// {{idx | safe}}
/// </div>
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct BlockMathHtmlTemplate {
    /// The rendered katex.
    pub content: String,
    /// The `1` based index!
    pub idx: u32,
    /// Set to true if the component did include an id so that the equation
    /// number can be shown accordingly.
    pub was_id: bool,
}
