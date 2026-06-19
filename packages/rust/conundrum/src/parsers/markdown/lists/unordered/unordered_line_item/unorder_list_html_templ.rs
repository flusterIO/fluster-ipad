use askama::Template;

/// ## Template (HTML)
/// ```askama
/// {% if let Some(body) = self.body %}
/// <li class="w-full [&>*]:inline">
/// <div class="prose-base">
/// {{heading | safe}}
/// </div>
/// <div class="prose-sm [&>p]:mt-2">
/// {{body | safe}}
/// </div>
/// </li>
/// {% else %}
/// <li class="w-full prose-base [&>*]:inline">
/// {{heading | safe}}
/// </li>
/// {% endif %}
/// ```
#[derive(Template, Debug)]
#[template(ext = "html", in_doc = true)]
pub struct UnorderedListItemHtmlTemplate {
    pub heading: String,
    pub body: Option<String>,
}

/// ## Template (HTML)
///
/// ```askama
/// <ul class="w-full my-4 max-w-[1080px] list-disc list-inside">
/// {% for item in items %}
/// {{item.render()? | safe}}
/// {% endfor %}
/// </ul>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct UnorderedListHtmlTemplate {
    pub items: Vec<UnorderedListItemHtmlTemplate>,
}
