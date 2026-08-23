use crate::ecosystem::error_handling::db_error::DatabaseResult;

pub trait IntoSettingValue {
    fn into_setting_value(&self) -> DatabaseResult<String>;
}

pub trait FromSettingString {
    fn from_setting_string(setting: String) -> DatabaseResult<Self>
        where Self: Sized;
}

