#[derive(serde::Serialize, serde::Deserialize, specta::Type, Clone, Debug, fake::Dummy)]
pub struct Hypothesis {
    /// A short label describing the theory in less than 80 characters
    pub label: String,
    /// The problem that the hypothesis is trying to solve.
    pub problem: String,
    /// A description of the theory.
    pub description: String,
}
