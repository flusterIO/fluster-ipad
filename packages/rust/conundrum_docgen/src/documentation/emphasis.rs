/// ## Template (Conundrum)
#[derive(askama::Template)]
#[template(ext = "jinja", filter = none, path = "/markdown/documentation/emphasis.txt")]
pub struct EmphasisDocs {}
