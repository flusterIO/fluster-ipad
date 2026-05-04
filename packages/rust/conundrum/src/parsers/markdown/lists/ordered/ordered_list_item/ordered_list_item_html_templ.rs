use askama::Template;

/// ## Template (HTML)
/// ```askama
/// {% if let Some(body) = self.body %}
/// <li class="w-full">
/// <div>
/// {{heading | safe}}
/// </div>
/// <div>
/// {{body | safe}}
/// </div>
/// </li>
/// {% else %}
/// <li class="w-full">
/// {{heading | safe}}
/// </li>
/// {% endif %}
/// ```
#[derive(Template, Debug)]
#[template(ext = "html", in_doc = true)]
pub struct OrderedListItemHtmlTemplate {
    /// The idx as it appears in the list, not necessarily the user provided
    /// number.
    pub idx: i32,
    pub heading: String,
    pub body: Option<String>,
}

/// ## Template (HTML)
///
/// ```askama
/// <ol class="w-full max-w-[1080px] list-disc">
/// {% for item in items %}
/// {{item.render()? | safe}}
/// {% endfor %}
/// </ol>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct OrderedListHtmlTemplate {
    pub items: Vec<OrderedListHtmlTemplate>,
}
