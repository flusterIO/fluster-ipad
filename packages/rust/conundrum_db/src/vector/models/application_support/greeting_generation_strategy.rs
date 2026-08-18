use conundrum::renamed_enum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, strum_macros::Display, strum_macros::EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum GreetingGenerationStrategy {
    /// Generate a greeting with AI, but keep it around for this many hours.
    Hours(u16),
    EveryTime,
    /// Don't generate greetings with AI at all. Save those tokens...
    Static,
}
