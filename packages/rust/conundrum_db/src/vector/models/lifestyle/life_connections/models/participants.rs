use crate::vector::models::lifestyle::life_connections::models::person::Person;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct Participants(Vec<Person>);

impl Default for Participants {
    fn default() -> Self {
        Self(Vec::new())
    }
}
