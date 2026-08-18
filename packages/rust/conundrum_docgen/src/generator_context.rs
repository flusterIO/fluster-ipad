use conundrum::ecosystem::db::tables::DatabaseTable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeneratorContext {}

impl Default for GeneratorContext {
    fn default() -> Self {
        Self {}
    }
}
