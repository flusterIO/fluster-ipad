use resolve_path::PathResolveExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{errors::config_error::ConfigError, utils::cwd::cwd};

/// FUCKKKKKK this figment library. The biggest mistake I made in a while but I
/// don't have internet to install something else and I don't want to implement
/// the environment overrides myself. I'm about to launch my computer out the
/// fucking window...
#[derive(Clone)]
pub struct ConfigPath {
    /// The value **exactly** as provided by the user.
    pub value: String,
    /// Path to the file the string originates from.
    pub source: PathBuf,
}

impl JsonSchema for ConfigPath {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("config-path")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        generator.root_schema_for::<String>()
    }
}

impl Serialize for ConfigPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
        serializer.serialize_str(&self.value)
    }
}

impl<'a> Deserialize<'a> for ConfigPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'a> {
        let value = String::deserialize(deserializer)?;
        let current_dir = std::env::current_dir().expect("Cannot continue without a valid current working directory.");

        Ok(ConfigPath { value,
                        source: current_dir })
    }
}

impl<'a> TryFrom<&'a str> for ConfigPath {
    type Error = ConfigError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let current_dir = cwd()?;
        Ok(ConfigPath { value: value.to_string(),
                        source: current_dir })
    }
}

impl ConfigPath {
    pub fn resolve(&self) -> PathBuf {
        let p = std::path::Path::new(&self.value);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            self.value.as_str().resolve_in(self.source.as_path()).to_path_buf()
        }
    }

    pub fn from_str_or_panic(value: &str) -> Self {
        Self::try_from(value).expect("Conundrum cannot continue. We can't even figure out where your root directory is. This is pretty embarrassing...")
    }
}
