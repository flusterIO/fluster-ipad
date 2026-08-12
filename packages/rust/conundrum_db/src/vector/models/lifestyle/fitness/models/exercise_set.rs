use crate::vector::models::{date_time::date_time::DateTime, primitives::db_id::DatabaseId};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct ExerciseSet {
    /// The id of the exercise in the database. AI should query this as needed
    /// to get more information about this exercise.
    pub exercise_id: DatabaseId,
    /// Any useful information regarding the performance of this set.
    /// If a user notes a nagging pain or otherwise mentions how they felt,
    /// mention that here so you can create more tailored suggestions over
    /// time.
    pub notes: Option<String>,
    #[serde(default = "DateTime::new_now")]
    pub performed_on: DateTime,
}
