use std::path::PathBuf;

use conundrum::{
    ecosystem::glue::conundrum_web_types::builder_output::next::NextjsFileSummary,
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};

#[derive(Clone)]
pub struct ProjectFileDescription {
    pub input_path: PathBuf,
    pub root_path: PathBuf,
    pub results: MdxParsingResult,
}

impl ProjectFileDescription {
    pub fn to_blog_summary(&self) -> NextjsFileSummary {
        NextjsFileSummary { html: self.results.content.clone(),
                            tags: self.results.tags.iter().map(|t| t.body.clone()).collect(),
                            relative_path: self.input_path
                                               .as_path()
                                               .strip_prefix(&self.root_path)
                                               .map(|p| String::from(p.to_str().unwrap_or_default()))
                                               .unwrap_or_default(),
                            front_matter: self.results.front_matter.clone() }
    }
}
