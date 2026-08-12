use std::fmt::Display;

use fake::Dummy;

/// # AI Generated Input
///
/// While AI is permitted to interact with the user's database in a variety of
/// ways, when the user permits, this `ai_generated_input` field is yours to
/// treat as your notepad.
///
/// Keep track of your interactions with each struct in a way that will help you
/// grow your experience with the user over time, and keep track of any
/// information provided by the user that will make future interactions more
/// meaningful.
///
/// It's a string, so store your notes here as markdown
/// or json as you prefer.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct AIGeneratedInput(String);

impl Default for AIGeneratedInput {
    fn default() -> Self {
        Self(String::new())
    }
}

impl Display for AIGeneratedInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
