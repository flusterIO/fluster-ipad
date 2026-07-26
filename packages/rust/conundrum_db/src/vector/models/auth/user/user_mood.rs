use conundrum::lang::runtime::traits::ai::ai_describable::AIDescribable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, strum_macros::Display)]
pub enum UserMood {
    /// I'm curious, help me explore new topics based on my recent habits and
    /// research.
    Curious,
    /// I'm focused, so I only want advice oriented at what's directly in front
    /// of me.
    Focused,
    /// I'm curious, but I want to be a little more free-flowing. Most apps
    /// should choose to turn down (up? I forgot which way is which) the
    /// temperature settings here to let the model be more creative.
    ///
    /// Users should be aware that this setting will likely affect accuracy in
    /// the way that many apps will choose to implement it.
    Creative,
    /// Help me organize! Summarize notes, re-organize to-do lists and help me
    /// get my act together.
    Scattered,
    /// Help me learn a new topic. Write new content, crawl the web for me,
    /// etc...
    Lost,
    /// This is a general purpose mood that may or may not be supported by
    /// applications that choose to adopt Conundrum. Any string should work, for
    /// best accuracy in the context that it'll be used it should be short.
    Other(String),
}

impl AIDescribable for UserMood {
    fn describe_self_for_ai(&self) -> String {
        match self {
            Self::Curious => {
                String::from("I'm curious, help me explore new topics based on my recent habits and research.")
            }
            Self::Focused => r#"
    I'm focused, so I only want advice oriented at what's directly in front of me.
    "#.to_string(),
            Self::Creative => r#"
     I'm curious, but I want to be a little more free-flowing. As AI that supports a generally strictly academic platform, this is your opportunity to be creative.
    "#.to_string(),
            Self::Scattered => r#"
    I'm a little scatterd, can you help me organize? Help me by focusing on summarizing notes, re-organizing to-do lists, and organizing the tasks in front of me into a coherent plan of action..
    "#.to_string(),
            Self::Lost => r#"
     I'm in the mood to learn something new! Help me explore new topics, but keep in mind my primary goals. Feel free to suggest new avenues of exploration, new techniques for problem solving, approaches the user hasn't yet mentioned.
    "#.to_string(),
            Self::Other(s) => format!("The user's current mood is {}", s),
        }
    }
}
