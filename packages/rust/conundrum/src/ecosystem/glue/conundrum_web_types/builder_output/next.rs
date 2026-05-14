use serde::{Deserialize, Serialize};

use crate::output::parsing_result::front_matter::FrontMatterResult;

#[typeshare::typeshare]
#[derive(Serialize, Deserialize)]
pub struct NextjsFileSummary {
    pub html: String,
    /// ## TODO:
    /// - [ ] Add a `keywords` field to the front-matter and access it here.
    pub keywords: Vec<String>,
    pub relative_path: String,
    pub front_matter: Option<FrontMatterResult>,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize)]
pub struct NextJsConundrumOutput {
    pub files: Vec<NextjsFileSummary>,
}
