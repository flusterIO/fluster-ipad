#[derive(serde::Serialize, serde::Deserialize, specta::Type, Clone, Debug, fake::Dummy)]
pub struct Concept {
    pub label: String,
    pub description: String,
}
