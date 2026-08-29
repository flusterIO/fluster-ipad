use crate::ecosystem::db::db_traits::db_field::DatabaseField;

#[derive(strum_macros::Display, serde::Serialize, serde::Deserialize, Clone, Debug, fake::Dummy, specta::Type)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum VectorMode {
    Local,
    Remote,
}

impl DatabaseField for VectorMode {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}
