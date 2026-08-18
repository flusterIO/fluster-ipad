#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
#[serde(tag = "strategy", content = "data", rename_all = "kebab-case")]
pub enum AIGeneratedSummaryStrategy {
    /// Update the content whenever Conundrum notices that the content is out of
    /// date. This strategy will keep your content most up to date at the
    /// expense of an increase in token expenditure.
    IfRequired,
    /// Only update the AI generated content when the user directly says so via
    /// a button click or other direct user input.
    DirectlyUserInitiated,
    /// Takes the number of *minutes* that the AI update should be 'debounced'.
    /// This means that if you provide a value of 20, Conundrum will wait at
    /// least 20 minutes since the last change to update the AI Generated
    /// summary. This type of behavior is ideal for fast changing workflows
    /// where you don't want to generate new vectors for every small change
    /// to the note that you're working on.  Rather, set this to the time
    /// you feel you'll be officially 'done' with your editing, and _then_
    /// Conundrum will generate vectors, summaries and other AI content.
    WithDebounce(f32),
}
