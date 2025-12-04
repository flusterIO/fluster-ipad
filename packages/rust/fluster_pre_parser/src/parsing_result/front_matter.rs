use gray_matter::Pod;
use serde::{Deserialize, Serialize};

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
pub struct FrontMatterResult {
    pub ignored_parsers: Vec<String>,
}

impl FrontMatterResult {
    fn get_ignore_parsers(data: &Pod) -> Vec<String> {
        let ignore_parsers = &data["ignore_parsers"];
        if let Ok(res) = ignore_parsers.as_vec() {
            res.iter().filter_map(|x| x.as_string().ok()).collect()
        } else {
            Vec::new()
        }
    }
    pub fn from_gray_matter(data: Pod) -> FrontMatterResult {
        FrontMatterResult {
            ignored_parsers: FrontMatterResult::get_ignore_parsers(&data),
        }
    }
}
