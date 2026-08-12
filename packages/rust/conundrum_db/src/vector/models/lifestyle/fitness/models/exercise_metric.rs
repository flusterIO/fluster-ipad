#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub enum ExerciseMetric {
    /// The weight applied to the exercise.
    Weight(f32),
    /// The elapsed time in seconds.
    Time(f32),
    /// The measure of length associated with the exercise: The height of a
    /// vertical leap or the length of a long-jump.
    Length(f32),
}
