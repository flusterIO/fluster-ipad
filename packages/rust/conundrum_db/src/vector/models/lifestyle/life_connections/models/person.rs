use crate::vector::models::lifestyle::life_connections::models::person_name_group::PersonNameGroup;
use crate::vector::models::lifestyle::life_connections::models::personal_relationship_type::PersonalRelationshipType;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct Person {
    pub name: PersonNameGroup,
    pub relationship: Option<PersonalRelationshipType>,
}
