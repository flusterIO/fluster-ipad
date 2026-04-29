use askama::Template;

/// ## Template (HTML)
///
/// ```askama
/// {% if let Some(style) = self.css %}<style>
/// {{style | safe}}
/// </style>{% endif %}
/// <div class="cdrm-body-container w-full h-fit" onresize="window.conundrum.onResize(event)"
/// onload="window.conundrum.onLoad()">
/// {{content | safe}}
/// </div>
/// <script defer>
/// {{js | safe}}
/// console.info("App glue code loaded...")
/// </script>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct AppAndAssetEmbedded {
    pub content: String,
    pub js: String,
    pub css: Option<String>,
}
