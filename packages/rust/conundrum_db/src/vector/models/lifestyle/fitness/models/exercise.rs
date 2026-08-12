use crate::vector::models::{
    ai::ai_generated_input::AIGeneratedInput, lifestyle::fitness::models::exercise_goal::ExerciseGoal,
    primitives::db_id::DatabaseId,
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct Exercise {
    pub id: DatabaseId,
    pub name: String,
    /// The goal the user is trying to reach by using this exercise
    pub goal: ExerciseGoal,
    pub ai: AIGeneratedInput,
}
