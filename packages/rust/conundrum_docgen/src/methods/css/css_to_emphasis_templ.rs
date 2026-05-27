use askama::Template;

#[derive(Template)]
#[template(ext = "rs", escape = "none", path = "emphasis_to_default_color_group.txt")]
pub struct CssToEmphasisTemplate {
    /// A list of properly css formatted theme names.
    pub theme_names: Vec<String>,
}
