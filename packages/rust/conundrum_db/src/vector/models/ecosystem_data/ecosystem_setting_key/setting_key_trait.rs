use crate::vector::models::ecosystem_data::ecosystem_setting::value_types::setting_value_type::SettingValueType;

pub trait EcosystemSettingKey {
    fn type_of_setting(&self) -> SettingValueType;
}
