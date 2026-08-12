#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub enum JournalEntrySentiment {
    /// Happy, cheerful and otherwise in a positive mood.
    Happy,
    /// Sad, depressed or a generally gloomy.
    Sad,
    /// The user is excited and anxious.
    Anxious,
    /// The user is angry or upset.
    Angry,
    /// The user is feeling alone and isolated.
    Lonely,
}
