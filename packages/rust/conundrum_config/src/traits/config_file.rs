use crate::errors::config_error::ConfigResult;
use clap_verbosity::{InfoLevel, LogLevel, Verbosity};
use figment_json5::Json5;

pub type Default_Verbosity = InfoLevel;

/// Any 'config' that can be reperesented as an independent file.
pub trait ConfigFile: Default {
    fn filename() -> &'static str;

    fn read(config_dir_path_override: &Option<String>) -> ConfigResult<Self>
        where Self: Sized {
        todo!();
    }
}
