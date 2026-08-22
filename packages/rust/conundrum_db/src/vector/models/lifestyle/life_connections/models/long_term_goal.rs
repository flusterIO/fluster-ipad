use conundrum_macros::DBSchema;

#[derive(DBSchema)]
pub struct LongTermGoal {
    /// Describe the user's goal in just a few words
    pub label: String,
    /// What is this user's long term goal?
    pub description: String,
}
