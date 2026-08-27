use conundrum::{
    ecosystem::db::{db_traits::db_field::DatabaseField, macros::renamed_enum},
    renamed_enum,
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, strum_macros::Display, strum_macros::EnumIter)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case", try_from = "String", into = "String")]
pub enum FrontMatterSourceType {
    Cdrm,
    Notebook,
}

renamed_enum!(FrontMatterSourceType);

impl DatabaseField for FrontMatterSourceType {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}
