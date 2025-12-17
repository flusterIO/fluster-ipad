use gray_matter::Pod;
use serde::{Deserialize, Serialize};

#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
pub struct FrontMatterResult {
    pub ignored_parsers: Vec<String>,
    pub title: Option<String>,
    pub user_defined_id: Option<String>,
}

impl FrontMatterResult {
    fn get_ignore_parsers(data: &mut Pod) -> Vec<String> {
        let ignore_parsers = data.remove("ignore_parsers".to_string());

        if ignore_parsers.is_empty() {
            Vec::new()
        } else {
            match ignore_parsers.as_vec() {
                Ok(res) => res.iter().filter_map(|x| x.as_string().ok()).collect(),
                Err(e) => {
                    println!("Front Matter Error: {}", e);
                    Vec::new()
                }
            }
        }
    }
    pub fn get_title(data: &mut Pod) -> Option<String> {
        let title = &data.remove("title".to_string());
        if title.is_empty() {
            None
        } else {
            match title.as_string() {
                Ok(s) => Some(s),
                Err(e) => {
                    println!("Front Matter Error: {}", e);
                    None
                }
            }
        }
    }
    pub fn get_user_defined_id(data: &mut Pod) -> Option<String> {
        let id = &data.remove("id".to_string());
        match id.as_string() {
            Ok(s) => Some(s),
            Err(e) => {
                println!("Front Matter Error: {}", e);
                None
            }
        }
    }
    pub fn from_gray_matter(data: Pod) -> FrontMatterResult {
        let mut p = data.clone();
        FrontMatterResult {
            ignored_parsers: FrontMatterResult::get_ignore_parsers(&mut p),
            title: FrontMatterResult::get_title(&mut p),
            user_defined_id: FrontMatterResult::get_user_defined_id(&mut p),
        }
    }
}
