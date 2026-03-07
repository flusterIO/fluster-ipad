use gray_matter::Pod;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

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

#[wasm_bindgen]
#[derive(uniffi::Record, Debug, Clone, Serialize, Deserialize)]
pub struct FrontMatterResult {
    pub(crate) ignored_parsers: Vec<String>,
    pub(crate) title: Option<String>,
    pub(crate) user_defined_id: Option<String>,
    pub(crate) file_path: Option<String>,
    pub(crate) topic: Option<String>,
    pub(crate) subject: Option<String>,
}

#[wasm_bindgen]
impl FrontMatterResult {
    /// CONSTRUCTOR: Create a new instance with default values.
    /// In a professional academic app, this allows you to initialize
    /// front matter before the parser even touches the file.
    #[wasm_bindgen(constructor)]
    pub fn new(
        ignored_parsers: Vec<String>,
        title: Option<String>,
        user_defined_id: Option<String>,
        file_path: Option<String>,
        topic: Option<String>,
        subject: Option<String>,
    ) -> Self {
        Self {
            ignored_parsers,
            title,
            user_defined_id,
            file_path,
            topic,
            subject,
        }
    }

    /// GETTER: Serializes the struct into a plain JavaScript object.
    /// Use this to pass metadata into your React components for rendering.
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.title).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen(setter)]
    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    #[wasm_bindgen(getter)]
    pub fn ignored_parsers(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.ignored_parsers).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen(setter)]
    pub fn set_ignored_parsers(&mut self, ignored_parsers: Vec<String>) {
        self.ignored_parsers = ignored_parsers;
    }

    pub fn get_user_defined_id_rust(self) -> Option<String> {
        self.user_defined_id
    }
    #[wasm_bindgen(getter)]
    pub fn user_defined_id(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.user_defined_id).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen(setter)]
    pub fn set_user_defined_id(&mut self, user_defined_id: String) {
        self.user_defined_id = Some(user_defined_id);
    }
    #[wasm_bindgen(getter)]
    pub fn file_path(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.file_path).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen(setter)]
    pub fn set_file_path(&mut self, file_path: String) {
        self.file_path = Some(file_path);
    }
    #[wasm_bindgen(getter)]
    pub fn topic(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.topic).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen(setter)]
    pub fn set_topic(&mut self, topic: String) {
        self.topic = Some(topic);
    }
    #[wasm_bindgen(getter)]
    pub fn subject(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.subject).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen(setter)]
    pub fn set_subject(&mut self, subject: String) {
        self.subject = Some(subject);
    }

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
                    println!("Front Matter Error ({}): {}", k, e);
                    Vec::new()
                }
            }
        }
    }
}

pub trait FrontMatterParser {
    fn from_gray_matter(data: Pod) -> FrontMatterResult;
}

impl FrontMatterParser for FrontMatterResult {
    fn from_gray_matter(data: Pod) -> FrontMatterResult {
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
