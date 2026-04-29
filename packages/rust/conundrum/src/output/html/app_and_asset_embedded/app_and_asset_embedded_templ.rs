use askama::Template;

/// ## Template (HTML)
///
/// ```askama
/// <div class="cdrm-body-container w-full h-fit" onresize="window.conundrum.onResize(event)"
/// onload="window.conundrum.onLoad()">
/// {{content | safe}}
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct AppAndAssetEmbedded {
    pub content: String,
}
