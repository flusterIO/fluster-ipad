use conundrum::ecosystem::error_handling::db_error::DatabaseResult;

use crate::vector::models::ecosystem_data::ecosystem_setting_key::unique_setting_key::UniqueSettingKey;

pub trait EcosystemSettingKey {
    fn to_setting_key(&self) -> UniqueSettingKey;
}
