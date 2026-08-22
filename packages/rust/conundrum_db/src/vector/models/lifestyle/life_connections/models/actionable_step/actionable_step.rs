pub struct ActionableStep {
    /// In just a few words, what can the user do to help them further their
    /// goal?
    pub label: String,
    /// You can be more verbose about this step. What can you and the user do in
    /// this phase to further the progress towards their ultimate goal?
    pub description: String,
    /// Is this task complete?
    pub complete: bool,
}
