use conundrum::ecosystem::db::db_traits::db_identifiable::DatabaseIdentifiable;
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Serialize,
           Deserialize,
           Clone,
           Debug,
           strum_macros::Display,
           strum_macros::EnumIter,
           strum_macros::EnumString,
           specta::Type,
           Dummy)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum UniqueSettingKey {
    FirstName,
    LastName,
    Profession,
    AutoSyncOnNewChat,
    AutoSyncOnNewMsg,
    LocalAiPreference,
    SaveLogDuration,
    LogVectorGenMethod,
}

impl DatabaseIdentifiable for UniqueSettingKey {
    fn to_predicate(&self, field_key: &str) -> String {
        format!("{} = \"{}\"", field_key, self)
    }
}
