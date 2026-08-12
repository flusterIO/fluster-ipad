use std::fmt::Display;

use fake::Dummy;

// # AI Notes
//
// These are the notes especially for AI that in most cases, come directly from
// the user. If a command is given here, it should override previous commands as
// input from the user should always come first.
//
// AI should **never** modify this field, but instead modify the associated
// `ai_generated_input` field.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct AINotes(String);

impl Default for AINotes {
    fn default() -> Self {
        AINotes(String::from(""))
    }
}

impl Display for AINotes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
