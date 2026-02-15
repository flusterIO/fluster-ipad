use gray_matter::Pod;
use serde::{Deserialize, Serialize};

#[derive(uniffi::Enum, strum_macros::Display, Clone, Serialize, Deserialize)]
pub enum FrontMatterKey {
    #[serde(rename = "ignoreParsers")]
    #[strum(to_string = "ignoreParsers")]
    IgnoreParsers,
    #[serde(rename = "title")]
    #[strum(to_string = "title")]
    Title,
    #[serde(rename = "id")]
    #[strum(to_string = "id")]
    Id,
    #[serde(rename = "topic")]
    #[strum(to_string = "topic")]
    Topic,
    #[serde(rename = "subject")]
    #[strum(to_string = "subject")]
    Subject,
    #[serde(rename = "filePath")]
    #[strum(to_string = "filePath")]
    FilePath,
}

#[derive(uniffi::Record, Debug, Clone, Serialize, Deserialize)]
pub struct FrontMatterResult {
    pub ignored_parsers: Vec<String>,
    pub title: Option<String>,
    pub user_defined_id: Option<String>,
    pub file_path: Option<String>,
    pub topic: Option<String>,
    pub subject: Option<String>,
}

impl FrontMatterResult {
    fn get_optional_string_by_key(data: &mut Pod, k: FrontMatterKey) -> Option<String> {
        let fp = &data.remove(k.to_string());
        if fp.is_empty() {
            None
        } else {
            match fp.as_string() {
                Ok(s) => Some(s),
                Err(e) => {
                    println!("Front Matter Error (user_defined_id): {}", e);
                    None
                }
            }
        }
    }
    fn get_string_vector(data: &mut Pod, k: FrontMatterKey) -> Vec<String> {
        let ignore_parsers = data.remove(k.to_string());

        if ignore_parsers.is_empty() {
            Vec::new()
        } else {
            match ignore_parsers.as_vec() {
                Ok(res) => res.iter().filter_map(|x| x.as_string().ok()).collect(),
                Err(e) => {
                    println!("Front Matter Error (ignored parsers): {}", e);
                    Vec::new()
                }
            }
        }
    }
    pub fn from_gray_matter(data: Pod) -> FrontMatterResult {
        let mut p = data.clone();
        FrontMatterResult {
            ignored_parsers: FrontMatterResult::get_string_vector(
                &mut p,
                FrontMatterKey::IgnoreParsers,
            ),
            subject: FrontMatterResult::get_optional_string_by_key(&mut p, FrontMatterKey::Subject),
            topic: FrontMatterResult::get_optional_string_by_key(&mut p, FrontMatterKey::Topic),
            file_path: FrontMatterResult::get_optional_string_by_key(
                &mut p,
                FrontMatterKey::FilePath,
            ),
            title: FrontMatterResult::get_optional_string_by_key(&mut p, FrontMatterKey::Title),
            user_defined_id: FrontMatterResult::get_optional_string_by_key(
                &mut p,
                FrontMatterKey::Id,
            ),
        }
    }
}
