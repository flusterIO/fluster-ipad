use serde::{Deserialize, Serialize};

/// ## AssignmentStatus
///
/// These statuses kind of intentionally replicate a typical project management
/// setup in a kanban board. Feel free to build your UI either replicating that
/// kanban board, or just ignore certain statuses and treat it as a boolean
/// indicating 'in-progress' or not... you do you.
#[derive(strum_macros::Display, Clone, Serialize, Deserialize)]
#[serde(tag = "tag", content = "content")]
pub enum AssignmentStatus {
    /// The initial status for a general task that's incomplete.
    #[strum(to_string = "to_do")]
    #[serde(rename = "to_do")]
    ToDo,
    /// When someone on the team, or yourself is handling the task.
    #[strum(to_string = "in_progress")]
    #[serde(rename = "in_progress")]
    InProgress,
    /// Building an app? This is for version two. Writing a big paper? This is
    /// for after the initial draft is done.
    #[strum(to_string = "up_next")]
    #[serde(rename = "up_next")]
    UpNext,
    /// The place for stuff with s--t in the way. Like if your friend Steve is
    /// slacking on something that Tracy needs to get her work done. Put
    /// Tracy's work here, and yell at Steve.
    #[strum(to_string = "on_hold")]
    #[serde(rename = "on_hold")]
    OnHold,
    /// The 'done' folder.
    #[strum(to_string = "complete")]
    #[serde(rename = "complete")]
    Complete,
    /// The trash bin basically.
    #[strum(to_string = "archived")]
    #[serde(rename = "archived")]
    Archived,
}
