use std::sync::Arc;

use conundrum::{
    ecosystem::{
        db::{db::ArcMutexDB, db_traits::db_identifiable::DatabaseIdentifiable},
        error_handling::db_error::DatabaseResult,
    },
    lang::lib::std_lib_impls::json_string::QuotedString,
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::ecosystem_data::{
    ecosystem_setting_key::setting_key::Setting, ecosytem_setting_types::ecosystem_setting_model::EcosystemSettingModel,
};

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
    AutoCleanVectors,
}

impl DatabaseIdentifiable for UniqueSettingKey {
    fn to_predicate(&self, field_key: &str) -> String {
        format!("{} = {}",
                field_key,
                self.to_string().to_quoted_string().unwrap_or(format!("\"{}\"", self.to_string())))
    }
}

impl UniqueSettingKey {
    pub async fn read_setting(&self, database: &ArcMutexDB) -> DatabaseResult<EcosystemSettingModel> {
        let setting = EcosystemSettingModel::get_by_setting_key(self.clone(), &Arc::clone(&database)).await?;
        Ok(match setting {
            Some(s) => s,
            None => {
                let data = Setting::from(self.clone());
                EcosystemSettingModel { key: self.clone(),
                                        data }
            }
        })
    }
}
