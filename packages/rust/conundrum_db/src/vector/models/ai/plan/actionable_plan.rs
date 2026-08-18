use conundrum::lifted_models::primitives::db_id::DatabaseId;

pub struct ActionablePlan {
    pub id: DatabaseId,
    /// The id of the associated `ShortTermGoal`.
    pub goal_id: DatabaseId,
}
