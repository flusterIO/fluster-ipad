use figment::{
    Figment,
    providers::{Format, Json, Toml, YamlExtended},
};
use serde::Deserialize;

use crate::errors::config_error::{ConfigError, ConfigResult};

/// Any 'config' that can be reperesented as an independent file.
pub trait ConfigFile {
    fn filename() -> &'static str;

    fn figment() -> Figment {
        let file_name = Self::filename();
        Figment::new().merge(Toml::file(file_name)).merge(Json::file(file_name)).merge(YamlExtended::file(file_name))
    }

    fn read<'a>() -> ConfigResult<Self>
        where Self: Sized + Deserialize<'a> {
        let f: Self = Self::figment().extract().map_err(|e| {
                                                    eprintln!("Error: {}", e);
                                                    ConfigError::OhShit
                                                })?;
        Ok(f)
    }
}
