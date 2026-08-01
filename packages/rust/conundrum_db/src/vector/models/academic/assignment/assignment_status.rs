use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::{fmt::Display, str::FromStr};

/// ## AssignmentStatus
///
/// These statuses kind of intentionally replicate a typical project management
/// setup in a kanban board. Feel free to build your UI either replicating that
/// kanban board, or just ignore certain statuses and treat it as a boolean
/// indicating 'in-progress' or not... you do you.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "tag", content = "content", try_from = "String")]
pub enum AssignmentStatus {
    /// The initial status for a general task that's incomplete.
    ToDo,
    /// When someone on the team, or yourself is handling the task.
    InProgress,
    /// Building an app? This is for version two. Writing a big paper? This is
    /// for after the initial draft is done.
    UpNext,
    /// The place for stuff with s--t in the way. Like if your friend Steve is
    /// slacking on something that Tracy needs to get her work done. Put
    /// Tracy's work here, and yell at Steve.
    OnHold,
    /// The 'done' folder.
    Complete,
    /// The trash bin basically.
    Archived,
    Custom(String),
}

impl Display for AssignmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::ToDo => "to_do",
            Self::InProgress => "in_progress",
            Self::UpNext => "up_next",
            Self::OnHold => "on_hold",
            Self::Complete => "complete",
            Self::Archived => "archived",
            Self::Custom(s) => s.as_str(),
        };
        write!(f, "{}", s)
    }
}

impl FromStr for AssignmentStatus {
    type Err = DatabaseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "to_do" => Self::ToDo,
            "in_progress" => Self::InProgress,
            "up_next" => Self::UpNext,
            "on_hold" => Self::OnHold,
            "complete" => Self::Complete,
            "archived" => Self::Archived,
            _ => Self::Custom(s.to_string()),
        })
    }
}

impl TryFrom<String> for AssignmentStatus {
    type Error = DatabaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(value.as_str())
    }
}
