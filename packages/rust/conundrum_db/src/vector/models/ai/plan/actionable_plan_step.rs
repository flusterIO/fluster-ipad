use conundrum::lifted_models::primitives::db_id::DatabaseId;

pub struct ActionablePlanStep {
    /// The id of the associated `ActionablePlan`.
    pub plan_id: DatabaseId,
    /// The 0 based index of the step in the list of steps that make up the
    /// entire `ActionablePlan`.
    pub step_idx: u32,
    /// This is a short summary of the step and what the user will see first.
    /// Make it short, concise and informative.
    pub label: String,
    /// This is where you can provide more details about this step. Let the user
    /// know exactly what you plan to accmplish in this step, how it ties in
    /// with the goal of this `ActionablePlan`.
    pub description: String,
}
