use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub enum ApplicationPermission {
    /// The ability to write to the file system, **through** the Conundrum
    /// api's.
    /// This will not allow the application to write to Conundrum files
    /// directly, although they can probably do that anyways if they're on
    /// your system, but will instead allow their data to
    /// overwrite conundrum files on your system making them a co-equal source
    /// of truth when it comes to your notes.
    /// If you prefer to keep your file system as the source of truth, do not
    /// add this to any applications. If you're perpetually online and
    /// aren't worried about syncying conflicts, then add this to multiple
    /// applications, and so long as there aren't updates in-between
    /// syncs, there shouldn't be any unexpected conflicts.
    ///
    /// Use the 'priority' field in the application data to prioritize one
    /// application over the other, which will be used for comparison when
    /// syncing if both applications provide updates.
    ManagedFileSytemWrite,
}
