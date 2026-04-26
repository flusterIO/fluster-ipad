use askama::Template;

/// ## Template (HTML)
/// ```askama
/// {% if let Some(body) = self.body %}
/// <li class="w-full">
/// <div>
/// {{heading}}
/// </div>
/// <div>
/// {{body}}
/// </div>
/// </li>
/// {% else %}
/// <li class="w-full">
/// {{heading}}
/// </li>
/// {% endif %}
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct UnorderedListItemHtmlTemplate {
    pub heading: String,
    pub body: Option<String>,
}

/// ## Template (HTML)
///
/// ```askama
/// <ul>
/// {% for item in items %}
/// {{item.render()?}}
/// {% endfor %}
/// </ul>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct UnorderedListHtmlTemplate {
    pub items: Vec<UnorderedListItemHtmlTemplate>,
}
