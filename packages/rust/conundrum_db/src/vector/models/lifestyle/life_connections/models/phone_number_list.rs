use crate::vector::models::lifestyle::life_connections::models::phone_contact::PhoneContact;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct PhoneContactList(pub Vec<PhoneContact>);
