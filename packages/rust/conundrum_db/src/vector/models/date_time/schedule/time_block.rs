use crate::vector::models::date_time::date_time::DateTime;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct TimeBlock {
    /// The time the event starts.
    pub start: DateTime,
    /// The time the event ends, or is projected to end.
    pub end: Option<DateTime>,
}
